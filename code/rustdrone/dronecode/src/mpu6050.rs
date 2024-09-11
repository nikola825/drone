use defmt::error;
use embassy_stm32::{
    i2c::{Error, ErrorInterruptHandler, EventInterruptHandler, I2c, Instance, RxDma, SclPin, SdaPin, TxDma}, interrupt, mode::{Async, Blocking}, time::Hertz, Peripheral, Peripherals
};
use zerocopy::{big_endian, FromBytes, FromZeroes, Unaligned};

pub struct MPU6050 {
    i2c: I2c<'static, Async>,
    gyro_range: GyroRange
}

mod mpu_constants {
    pub const MPU_ADDRESS: u8 = 0x68;
    pub const I2C_FREQUENCY: u32 = 400_000;
    pub const DEVICE_ID: u8 = 0x68;

    pub const PWR_MGMT_1_CKLSEL_8MHZ_INTERNAL: u8 = 0;
    pub const PWR_MGMT_1_CKLSEL_X_GYRO_PLL: u8 = 1;

    pub const ACCEL_CONFIG_AFS_SEL_2: u8 = 0<<3;
    pub const ACCEL_CONFIG_AFS_SEL_4: u8 = 1<<3;
    pub const ACCEL_CONFIG_AFS_SEL_8: u8 = 2<<3;
    pub const ACCEL_CONFIG_AFS_SEL_16: u8 = 3<<3;
}

pub enum GyroRange {
    GYRO_CONFIG_FS_SEL_250 = 0<<3,
    GYRO_CONFIG_FS_SEL_500 = 1<<3,
    GYRO_CONFIG_FS_SEL_1000 = 2<<3,
    GYRO_CONFIG_FS_SEL_2000 = 3<<3
}

mod mpu_registers {
    pub const WHO_AM_I: u8 = 0x75;
    pub const PWR_MGMT_1: u8 = 0x6b;
    pub const GYRO_CONFIG: u8 = 0x1b;
    pub const ACCEL_CONFIG: u8 = 0x1c;
    pub const SMPLRT_DIV: u8 = 0x19;
    pub const DLPF_CFG: u8 = 0x1a;
    pub const GYRO_XOUT_H: u8 = 0x43;
}

#[derive(Default, FromBytes, FromZeroes)]
struct GyroOutputPack {
    x_out: big_endian::I16,
    y_out: big_endian::I16,
    z_out: big_endian::I16
}

impl GyroOutputPack {
    fn get_ypr_deg(&self, range: &GyroRange) -> (f32, f32, f32) {
        let divisor = match range {
            GyroRange::GYRO_CONFIG_FS_SEL_250 => 131f32,
            GyroRange::GYRO_CONFIG_FS_SEL_500 => 65.5f32,
            GyroRange::GYRO_CONFIG_FS_SEL_1000 => 32.8f32,
            GyroRange::GYRO_CONFIG_FS_SEL_2000 => 16.4f32,
        };

        let yaw = (self.z_out.get() as f32) / divisor;
        let pitch = (self.x_out.get() as f32) / divisor;
        let roll = (self.y_out.get() as f32) / divisor;

        return (yaw, pitch, roll);
    }
}

impl MPU6050 {
    pub fn new<T: Instance>(
        i2c_controller: impl Peripheral<P = T> + 'static,
        scl_pin: impl Peripheral<P = impl SclPin<T>> + 'static,
        sda_pin: impl Peripheral<P = impl SdaPin<T>> + 'static,
        irqs: impl interrupt::typelevel::Binding<T::EventInterrupt, EventInterruptHandler<T>>
        + interrupt::typelevel::Binding<T::ErrorInterrupt, ErrorInterruptHandler<T>>
        + 'static,
        tx_dma: impl Peripheral<P = impl TxDma<T>> + 'static,
        rx_dma: impl Peripheral<P = impl RxDma<T>> + 'static,
    ) -> MPU6050 {
        
        let i2c: I2c<Async> =
            I2c::new(i2c_controller, scl_pin, sda_pin, irqs, tx_dma, rx_dma, Hertz(mpu_constants::I2C_FREQUENCY), Default::default());

        /*let i2c: I2c<Blocking> =
            I2c::new_blocking(i2c_controller, scl_pin, sda_pin,  Hertz(mpu_constants::I2C_FREQUENCY), Default::default());*/

        MPU6050 { i2c: i2c, gyro_range: GyroRange::GYRO_CONFIG_FS_SEL_250 }
    }

    pub fn test_connection(&mut self) -> bool{
        let mut data = [0u8];
        match self.i2c.blocking_write_read(mpu_constants::MPU_ADDRESS, &[mpu_registers::WHO_AM_I], &mut data) {
            Ok(()) => {
                return data[0]==mpu_constants::DEVICE_ID;
            }
            Err(Error::Timeout) => {
                error!("MPU I2C timeout");
                return false;
            },
            Err(e) => {
                error!("MPU I2C Error: {:?}", e);
                return false;
            }
        }
    }

    async fn write_reg_byte(&mut self, reg_addr: u8, reg_data: u8) {
        self.i2c.write(mpu_constants::MPU_ADDRESS, &[reg_addr, reg_data]).await.unwrap();
    }

    async fn read_reg_bytes(&mut self, reg_addr: u8, reg_data: &mut [u8]) {
        self.i2c.write_read(mpu_constants::MPU_ADDRESS, &[reg_addr], reg_data).await.unwrap();
    }

    pub async fn init(&mut self, gyro_range:GyroRange){
        self.write_reg_byte(mpu_registers::PWR_MGMT_1, mpu_constants::PWR_MGMT_1_CKLSEL_X_GYRO_PLL).await;
        self.write_reg_byte(mpu_registers::GYRO_CONFIG, gyro_range as u8).await;
        self.write_reg_byte(mpu_registers::ACCEL_CONFIG, mpu_constants::ACCEL_CONFIG_AFS_SEL_2).await;
        self.write_reg_byte(mpu_registers::DLPF_CFG, 0).await;
        self.write_reg_byte(mpu_registers::SMPLRT_DIV, 0).await;
    }

    pub async fn get_ypr_deg(&mut self) -> (f32, f32, f32) {
        let mut data = [0u8;6];
        self.read_reg_bytes(mpu_registers::GYRO_XOUT_H, &mut data).await;

        GyroOutputPack::ref_from(&data).unwrap().get_ypr_deg(&self.gyro_range)
    }
}
