use embassy_stm32::{
    gpio::{Output, Pin},
    mode::Async,
    spi::{self, BitOrder, Instance, MisoPin, MosiPin, RxDma, SckPin, Spi, TxDma},
    time::Hertz,
    Peripheral,
};
use embassy_time::Timer;
use embedded_hal::spi::MODE_3;
use zerocopy::{big_endian, FromBytes, FromZeroes};

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum GyroFSRange {
    Dps2000 = 0b000 << 5,
    Dps1000 = 0b001 << 5,
    Dps500 = 0b010 << 5,
    Dps250 = 0b011 << 5,
    Dps125 = 0b100 << 5,
    Dps62 = 0b101 << 5,
    Dps31 = 0b110 << 5,
    Dps15 = 0b111 << 5,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum GyroOutputDataRate {
    Odr1KHz = 0b0110,
    Odr100Hz = 0b1000,
}

pub struct ICM42688 {
    spi: Spi<'static, Async>,
    cs_pin: Output<'static>,
    gyro_fs_range: GyroFSRange,
    gyro_output_rate: GyroOutputDataRate,
}

#[allow(dead_code)]
mod icm_constants {
    pub const DEVICE_ID: u8 = 0x47;
    pub const PWR_MGMT0_ACCEL_LN: u8 = 0x03;
    pub const PWR_MGMT0_GYRO_LN: u8 = 0x0c;
}

#[allow(dead_code, non_camel_case_types)]
enum ICM42688Register {
    WHO_AM_I = 0x75,
    PWR_MGMT0 = 0x4e,
    GYRO_START = 0x25,
    GYRO_CONFIG0 = 0x4f,
}

#[derive(Default, FromBytes, FromZeroes)]
struct GyroOutputPack {
    x_out: big_endian::I16,
    y_out: big_endian::I16,
    z_out: big_endian::I16,
}

impl GyroOutputPack {
    fn get_ypr_deg(&self, range: GyroFSRange) -> (f32, f32, f32) {
        let divisor = match range {
            GyroFSRange::Dps2000 => 16.4f32,
            GyroFSRange::Dps1000 => 32.8f32,
            GyroFSRange::Dps500 => 65.5f32,
            GyroFSRange::Dps250 => 131f32,
            GyroFSRange::Dps125 => 262f32,
            GyroFSRange::Dps62 => 524.3f32,
            GyroFSRange::Dps31 => 1048.6f32,
            GyroFSRange::Dps15 => 2097.2f32,
        };

        (
            self.z_out.get() as f32 / divisor,
            self.x_out.get() as f32 / divisor,
            self.y_out.get() as f32 / divisor,
        )
    }
}

impl ICM42688 {
    pub fn new<T: Instance>(
        spi_controller: impl Peripheral<P = T> + 'static,
        sck_pin: impl SckPin<T> + 'static,
        mosi_pin: impl MosiPin<T> + 'static,
        miso_pin: impl MisoPin<T> + 'static,
        tx_dma: impl TxDma<T> + 'static,
        rx_dma: impl RxDma<T> + 'static,
        cs_pin: impl Pin + 'static,
    ) -> ICM42688 {
        let mut spi_config = spi::Config::default();
        spi_config.frequency = Hertz(20_000_000);
        spi_config.mode = MODE_3;
        spi_config.bit_order = BitOrder::MsbFirst;
        let spi = spi::Spi::new(
            spi_controller,
            sck_pin,
            mosi_pin,
            miso_pin,
            tx_dma,
            rx_dma,
            spi_config,
        );

        ICM42688 {
            spi: spi,
            cs_pin: Output::new(
                cs_pin,
                embassy_stm32::gpio::Level::High,
                embassy_stm32::gpio::Speed::VeryHigh,
            ),
            gyro_output_rate: GyroOutputDataRate::Odr1KHz,
            gyro_fs_range: GyroFSRange::Dps15,
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

        return data[0] == icm_constants::DEVICE_ID;
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
    }

    pub fn get_ypr_deg(&mut self) -> (f32, f32, f32) {
        let mut data = [0u8; 6];
        self.read_registers(ICM42688Register::GYRO_START, &mut data);

        GyroOutputPack::ref_from(&data)
            .unwrap()
            .get_ypr_deg(self.gyro_fs_range)
    }
}
