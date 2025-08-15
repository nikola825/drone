use embassy_stm32::{
    gpio::Output,
    mode::Async,
    spi::{BitOrder, Spi, MODE_3},
    time::Hertz,
};
use embassy_time::Timer;
use nalgebra::Vector3;
use zerocopy::{big_endian, FromBytes, Immutable, KnownLayout};

use crate::{hal::SpiMaker, logging::info, stored_config::StoredConfig};

// Gyro results will be multiplied with this vector to align controller inputs with orientation
// Expected gyro outputs are:
// - yaw (z): left-negative, right-positive
// - pitch (x) - forward-positive, backward-negative
// - roll (y) - left-negative, right-positive
pub const IMU_ORIENTATION_MULTIPLIER: Vector3<f32> = Vector3::new(1f32, -1f32, -1f32);

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

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy)]
enum ACCEL_FS_SEL {
    G_16 = 0b000 << 5,
    G_8 = 0b001 << 5,
    G_4 = 0b010 << 5,
    G_2 = 0b011 << 5,
}

#[allow(non_camel_case_types, dead_code, clippy::enum_variant_names)]
#[derive(Clone, Copy)]
enum GYRO_ODR {
    ODR_2KHz = 0b0101,
    ODR_1KHz = 0b0110,
    ODR_100Hz = 0b1000,
}

#[allow(non_camel_case_types, dead_code, clippy::enum_variant_names)]
#[derive(Clone, Copy)]
enum ACCEL_ODR {
    ODR_2KHz = 0b0101,
    ODR_1KHz = 0b0110,
    ODR_100Hz = 0b1000,
}

pub struct ICM42688 {
    spi: Spi<'static, Async>,
    cs_pin: Output<'static>,
    gyro_fs_range: GYRO_FS_SEL,
    gyro_output_rate: GYRO_ODR,
    accel_output_rate: ACCEL_ODR,
    accel_fs_range: ACCEL_FS_SEL,
}

#[allow(dead_code)]
mod icm_constants {
    pub const DEVICE_ID: u8 = 0x47;
    pub const PWR_MGMT0_ACCEL_LN: u8 = 0x03;
    pub const PWR_MGMT0_GYRO_LN: u8 = 0x0c;
    pub const GYRO_DEC2_M2_ORD: u8 = 0b10;
    pub const ACCEL_DEC2_M2_ORD: u8 = 0b10 << 1;

    pub const GYRO_UI_FILT_ORD1: u8 = 0b00 << 2;
    pub const GYRO_UI_FILT_ORD2: u8 = 0b01 << 2;
    pub const GYRO_UI_FILT_ORD3: u8 = 0b10 << 2;

    pub const ACCEL_UI_FILT_ORD1: u8 = 0b00 << 3;
    pub const ACCEL_UI_FILT_ORD2: u8 = 0b01 << 3;
    pub const ACCEL_UI_FILT_ORD3: u8 = 0b10 << 3;
}

#[allow(dead_code, non_camel_case_types)]
enum ICM42688Register {
    WHO_AM_I = 0x75,
    PWR_MGMT0 = 0x4e,
    ACCEL_START = 0x1f,
    GYRO_START = 0x25,
    GYRO_CONFIG0 = 0x4f,
    ACCEL_CONFIG0 = 0x50,
    GYRO_CONFIG1 = 0x51,
    GYRO_ACCEL_CONFIG0 = 0x52,
    ACCEL_CONFIG1 = 0x53,
    BANK_SEL = 0x76,
    GYRO_CONFIG_STATIC10 = 0x13,
}

#[derive(Default, FromBytes, KnownLayout, Immutable)]
struct GyroOutputPack {
    x_out: big_endian::I16,
    y_out: big_endian::I16,
    z_out: big_endian::I16,
}

#[derive(Default, FromBytes, KnownLayout, Immutable)]
struct AccelOutputPack {
    x_out: big_endian::I16,
    y_out: big_endian::I16,
    z_out: big_endian::I16,
}

impl GyroOutputPack {
    fn get_rotation_degrees(&self, range: GYRO_FS_SEL) -> Vector3<f32> {
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

        (Vector3::new(
            self.x_out.get() as f32,
            self.y_out.get() as f32,
            self.z_out.get() as f32,
        ) / divisor)
            .component_mul(&IMU_ORIENTATION_MULTIPLIER)
    }
}

impl AccelOutputPack {
    fn get_accel_g_zxy(&self, range: ACCEL_FS_SEL) -> (f32, f32, f32) {
        use ACCEL_FS_SEL::*;
        let divisor = match range {
            G_16 => 2048f32,
            G_8 => 4096f32,
            G_4 => 8192f32,
            G_2 => 16384f32,
        };

        (
            self.z_out.get() as f32 / divisor,
            self.x_out.get() as f32 / divisor,
            self.y_out.get() as f32 / divisor,
        )
    }
}

#[derive(Default, FromBytes, KnownLayout, Immutable)]
struct AccelGyroOutputPack {
    accel: AccelOutputPack,
    gyro: GyroOutputPack,
}

pub struct MotionData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_yaw: f32,
    pub gyro_pitch: f32,
    pub gyro_roll: f32,
}

impl MotionData {
    pub fn apply_gyro_offsets(self, yaw_offset: f32, pitch_offset: f32, roll_offset: f32) -> Self {
        Self {
            accel_x: self.accel_x,
            accel_y: self.accel_y,
            accel_z: self.accel_z,
            gyro_yaw: self.gyro_yaw - yaw_offset,
            gyro_pitch: self.gyro_pitch - pitch_offset,
            gyro_roll: self.gyro_roll - roll_offset,
        }
    }
}

impl ICM42688 {
    pub fn new(spi_maker: impl SpiMaker) -> ICM42688 {
        let (cs_pin, spi) = spi_maker.make_spi(Hertz(20_000_000), MODE_3, BitOrder::MsbFirst);

        ICM42688 {
            spi,
            cs_pin,
            gyro_output_rate: GYRO_ODR::ODR_1KHz,
            gyro_fs_range: GYRO_FS_SEL::DPS_2000,
            accel_fs_range: ACCEL_FS_SEL::G_16,
            accel_output_rate: ACCEL_ODR::ODR_1KHz,
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
        let accel_config = self.accel_fs_range as u8 | self.accel_output_rate as u8;
        self.write_register(ICM42688Register::GYRO_CONFIG0, gyro_config);
        self.write_register(ICM42688Register::ACCEL_CONFIG0, accel_config);

        self.write_register(
            ICM42688Register::PWR_MGMT0,
            icm_constants::PWR_MGMT0_GYRO_LN | icm_constants::PWR_MGMT0_ACCEL_LN,
        );

        Timer::after_millis(100).await;
        self.setup_filters();
        Timer::after_millis(100).await;
    }

    fn setup_filters(&mut self) {
        let gyro_filter_config = icm_constants::GYRO_UI_FILT_ORD1 | icm_constants::GYRO_DEC2_M2_ORD;
        self.write_register(ICM42688Register::GYRO_CONFIG1, gyro_filter_config);

        let accel_filter_config =
            icm_constants::ACCEL_UI_FILT_ORD1 | icm_constants::ACCEL_DEC2_M2_ORD;
        self.write_register(ICM42688Register::ACCEL_CONFIG1, accel_filter_config);

        self.write_register(ICM42688Register::GYRO_ACCEL_CONFIG0, 0x44);
    }

    #[allow(dead_code)]
    fn setup_notch(&mut self) {
        self.write_register(ICM42688Register::BANK_SEL, 1u8);
        self.write_register(ICM42688Register::GYRO_CONFIG_STATIC10, 7u8 << 3);
        self.write_register(ICM42688Register::BANK_SEL, 0u8);
    }

    pub fn get_motion_data(&mut self) -> MotionData {
        let mut data = [0u8; 12];
        self.read_registers(ICM42688Register::ACCEL_START, &mut data);

        let packet = AccelGyroOutputPack::ref_from_bytes(&data).unwrap();

        let gyro_ypr = packet.gyro.get_rotation_degrees(self.gyro_fs_range);
        let (accel_z, accel_x, accel_y) = packet.accel.get_accel_g_zxy(self.accel_fs_range);

        MotionData {
            accel_x,
            accel_y,
            accel_z,
            gyro_yaw: gyro_ypr.z,
            gyro_pitch: gyro_ypr.x,
            gyro_roll: gyro_ypr.y,
        }
    }
}

#[allow(dead_code)]
pub async fn calibrate_gyro_offsets(
    mut imu: ICM42688,
    config: &StoredConfig,
    apply_existing_offsets: bool,
) -> ! {
    const SMOOTHING_FACTOR: f32 = 0.001;

    let mut yaw_accumulated = 0f32;
    let mut pitch_accumulated = 0f32;
    let mut roll_accumulated = 0f32;

    loop {
        for _ in 0..1000 {
            let mut data = imu.get_motion_data();
            if apply_existing_offsets {
                data = data.apply_gyro_offsets(
                    config.yaw_offset.into(),
                    config.pitch_offset.into(),
                    config.roll_offset.into(),
                );
            }
            yaw_accumulated =
                yaw_accumulated * (1f32 - SMOOTHING_FACTOR) + data.gyro_yaw * SMOOTHING_FACTOR;
            pitch_accumulated =
                pitch_accumulated * (1f32 - SMOOTHING_FACTOR) + data.gyro_pitch * SMOOTHING_FACTOR;
            roll_accumulated =
                roll_accumulated * (1f32 - SMOOTHING_FACTOR) + data.gyro_roll * SMOOTHING_FACTOR;
            Timer::after_micros(1005).await;
        }

        info!("---------------------------");
        info!("config.yaw_offset = ({}).into();", yaw_accumulated);
        info!("config.pitch_offset = ({}).into();", pitch_accumulated);
        info!("config.roll_offset = ({}).into();", roll_accumulated);
    }
}
