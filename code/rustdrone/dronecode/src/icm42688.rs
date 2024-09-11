use defmt::{error, info};
use embassy_stm32::{
    gpio::Output, interrupt, mode::{Async, Blocking}, spi::{self, Instance, MisoPin, MosiPin, RxDma, SckPin, Spi, TxDma}, time::Hertz, Peripheral, Peripherals
};
use embedded_hal::spi::MODE_3;
use zerocopy::{big_endian, little_endian, FromBytes, FromZeroes, Unaligned};

pub struct ICM42688 {
    spi: Spi<'static, Async>,
    ss: Output<'static>,
    gyro_range: GyroRange
}

mod icm_constants {
    pub const DEVICE_ID: u8 = 211;
}

pub enum GyroRange {
    GYRO_CONFIG_FS_SEL_250 = 0<<3
}

mod icm_registers {
    pub const WHO_AM_I: u8 = 0x0f;
    pub const GYRO_REGISTERS: u8 = 0x28;
    pub const CTRL_REG1: u8 = 0x20;
    pub const CTRL_REG2: u8 = 0x21;
    pub const CTRL_REG3: u8 = 0x22;
    pub const CTRL_REG4: u8 = 0x23;
    pub const CTRL_REG5: u8 = 0x24;
}

#[derive(Default, FromBytes, FromZeroes)]
struct GyroOutputPack {
    x_out: little_endian::I16,
    y_out: little_endian::I16,
    z_out: little_endian::I16
}

impl GyroOutputPack {
    fn get_ypr_deg(&self, range: &GyroRange) -> (f32, f32, f32) {
        let divisor = match range {
            GyroRange::GYRO_CONFIG_FS_SEL_250 => 114.28571428571428f32
        };

        let yaw = (self.z_out.get() as f32) / divisor;
        let pitch = (self.x_out.get() as f32) / divisor;
        let roll = (self.y_out.get() as f32) / divisor;

        return (yaw, pitch, roll);
    }
}

impl ICM42688 {
    pub fn new<T: Instance>(
        spi_controller: impl Peripheral<P = T> + 'static,
        sck_pin: impl Peripheral<P = impl SckPin<T>> + 'static,
        mosi_pin: impl Peripheral<P = impl MosiPin<T>> + 'static,
        miso_pin: impl Peripheral<P = impl MisoPin<T>> + 'static,
        tx_dma: impl Peripheral<P = impl TxDma<T>> + 'static,
        rx_dma: impl Peripheral<P = impl RxDma<T>> + 'static,
        ss: Output<'static>
    ) -> ICM42688 {
        let mut spi_config = spi::Config::default();
        spi_config.frequency = Hertz(10_000_000);
        spi_config.mode = MODE_3;
        let spi = spi::Spi::new(spi_controller,sck_pin, mosi_pin, miso_pin, tx_dma, rx_dma, spi_config);

        ICM42688 { spi: spi, gyro_range: GyroRange::GYRO_CONFIG_FS_SEL_250, ss:ss }
    }

    pub fn read_registers(&mut self, address: u8, output: &mut [u8]) {
        self.ss.set_low();
        let mut data = 0x80u8 | address;
        if output.len() > 1 {
            data |= 0x40u8;
        }
        self.spi.blocking_write(&[data]).unwrap();
        self.spi.blocking_read(output).unwrap();
        self.ss.set_high();
    }

    pub fn write_registers(&mut self, address: u8, data: &[u8]) {
        self.ss.set_low();
        let mut start_byte = address;
        if data.len() > 1 {
            start_byte |= 0x40u8;
        }
        self.spi.blocking_write(&[start_byte]).unwrap();
        self.spi.blocking_write(data).unwrap();
        self.ss.set_high();
    }

    pub fn test_connection(&mut self) -> bool{
        let mut data = [0u8];
        self.read_registers(icm_registers::WHO_AM_I, &mut data);

        return data[0]==icm_constants::DEVICE_ID;
    }

    /*async fn write_reg_byte(&mut self, reg_addr: u8, reg_data: u8) {
        self.i2c.write(mpu_constants::MPU_ADDRESS, &[reg_addr, reg_data]).await.unwrap();
    }

    async fn read_reg_bytes(&mut self, reg_addr: u8, reg_data: &mut [u8]) {
        self.i2c.write_read(mpu_constants::MPU_ADDRESS, &[reg_addr], reg_data).await.unwrap();
    }*/

    pub fn init(&mut self, gyro_range:GyroRange){
        self.write_registers(0x20u8, &[0xffu8]);
    }
    
    pub fn get_ypr_deg(&mut self) -> (f32, f32, f32) {
        let mut data = [0u8;6];
        self.read_registers(icm_registers::GYRO_REGISTERS, &mut data);

        GyroOutputPack::ref_from(&data).unwrap().get_ypr_deg(&self.gyro_range)
    }
}
