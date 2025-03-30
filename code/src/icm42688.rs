use embassy_stm32::{
    gpio::Output,
    mode::Async,
    spi::{BitOrder, Spi, MODE_3},
    time::Hertz,
};
use embassy_time::Timer;
use zerocopy::{big_endian, FromBytes, Immutable, KnownLayout};

use crate::hw_select::SpiMaker;

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy)]
enum GYRO_FS_SEL {
    DPS_2000 = 0b000 << 5,
    DPS_1000 = 0b001 << 5,
    DPS_500 = 0b010 << 5,
    DPS_250 = 0b011 << 5,
    DPS_125 = 0b100 << 5,
    DPS_62_5 = 0b101 << 5,
    DPS_31_25 = 0b110 << 5,
    DPS_15_625 = 0b111 << 5,
}

#[allow(non_camel_case_types, dead_code, clippy::enum_variant_names)]
#[derive(Clone, Copy)]
enum GYRO_ODR {
    ODR_2KHz = 0b0101,
    ODR_1KHz = 0b0110,
    ODR_100Hz = 0b1000,
}

pub struct ICM42688 {
    spi: Spi<'static, Async>,
    cs_pin: Output<'static>,
    gyro_fs_range: GYRO_FS_SEL,
    gyro_output_rate: GYRO_ODR,
}

#[allow(dead_code)]
mod icm_constants {
    pub const DEVICE_ID: u8 = 0x47;
    pub const PWR_MGMT0_ACCEL_LN: u8 = 0x03;
    pub const PWR_MGMT0_GYRO_LN: u8 = 0x0c;
    pub const GYRO_DEC2_M2_ORD: u8 = 0b10;

    pub const GYRO_UI_FILT_ORD1: u8 = 0b00 << 2;
    pub const GYRO_UI_FILT_ORD2: u8 = 0b01 << 2;
    pub const GYRO_UI_FILT_ORD3: u8 = 0b10 << 2;
}

#[allow(dead_code, non_camel_case_types)]
enum ICM42688Register {
    WHO_AM_I = 0x75,
    PWR_MGMT0 = 0x4e,
    GYRO_START = 0x25,
    GYRO_CONFIG0 = 0x4f,
    GYRO_CONFIG1 = 0x51,
    GYRO_ACCEL_CONFIG0 = 0x52,
    BANK_SEL = 0x76,
    GYRO_CONFIG_STATIC10 = 0x13,
}

#[derive(Default, FromBytes, KnownLayout, Immutable)]
struct GyroOutputPack {
    x_out: big_endian::I16,
    y_out: big_endian::I16,
    z_out: big_endian::I16,
}

impl GyroOutputPack {
    fn get_ypr_deg(&self, range: GYRO_FS_SEL) -> (f32, f32, f32) {
        use GYRO_FS_SEL::*;
        let divisor = match range {
            DPS_2000 => 16.4f32,
            DPS_1000 => 32.8f32,
            DPS_500 => 65.5f32,
            DPS_250 => 131f32,
            DPS_125 => 262f32,
            DPS_62_5 => 524.3f32,
            DPS_31_25 => 1048.6f32,
            DPS_15_625 => 2097.2f32,
        };

        (
            self.z_out.get() as f32 / divisor,
            self.x_out.get() as f32 / divisor,
            self.y_out.get() as f32 / divisor,
        )
    }
}

impl ICM42688 {
    pub fn new(spi_maker: impl SpiMaker) -> ICM42688 {
        let (cs_pin, spi) = spi_maker.make_spi(Hertz(20_000_000), MODE_3, BitOrder::MsbFirst);

        ICM42688 {
            spi,
            cs_pin,
            gyro_output_rate: GYRO_ODR::ODR_1KHz,
            gyro_fs_range: GYRO_FS_SEL::DPS_1000,
        }
    }

    fn read_registers(&mut self, register: ICM42688Register, output: &mut [u8]) {
        self.cs_pin.set_low();
        let address = register as u8;
        let address: u8 = 0x80u8 | address;
        self.spi.blocking_write(&[address]).unwrap();
        self.spi.blocking_read(output).unwrap();
        self.cs_pin.set_high();
    }

    pub fn test_connection(&mut self) -> bool {
        let mut data = [0u8];
        self.read_registers(ICM42688Register::WHO_AM_I, &mut data);

        data[0] == icm_constants::DEVICE_ID
    }

    fn write_register(&mut self, register: ICM42688Register, reg_data: u8) {
        self.cs_pin.set_low();
        let address = register as u8;
        self.spi.blocking_write(&[address, reg_data]).unwrap();
        self.cs_pin.set_high();
    }

    pub async fn init(&mut self) {
        if !self.test_connection() {
            panic!("IMU connection failed");
        }
        let gyro_config = self.gyro_output_rate as u8 | self.gyro_fs_range as u8;
        self.write_register(ICM42688Register::GYRO_CONFIG0, gyro_config);

        self.write_register(
            ICM42688Register::PWR_MGMT0,
            icm_constants::PWR_MGMT0_GYRO_LN,
        );

        Timer::after_millis(100).await;
        self.setup_filters();
        Timer::after_millis(100).await;
    }

    fn setup_filters(&mut self) {
        let filter_config = icm_constants::GYRO_UI_FILT_ORD1 | icm_constants::GYRO_DEC2_M2_ORD;
        self.write_register(ICM42688Register::GYRO_CONFIG1, filter_config);

        self.write_register(ICM42688Register::GYRO_ACCEL_CONFIG0, 0x44);
    }

    #[allow(dead_code)]
    fn setup_notch(&mut self) {
        self.write_register(ICM42688Register::BANK_SEL, 1u8);
        self.write_register(ICM42688Register::GYRO_CONFIG_STATIC10, 7u8 << 3);
        self.write_register(ICM42688Register::BANK_SEL, 0u8);
    }

    pub fn get_ypr_deg(&mut self) -> (f32, f32, f32) {
        let mut data = [0u8; 6];
        self.read_registers(ICM42688Register::GYRO_START, &mut data);

        GyroOutputPack::ref_from_bytes(&data)
            .unwrap()
            .get_ypr_deg(self.gyro_fs_range)
    }
}
