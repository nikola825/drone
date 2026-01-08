use embassy_executor::SendSpawner;
use embassy_stm32::{
    gpio::AnyPin,
    usb::{DmPin, DpPin},
    Peri,
};

mod config_storage;
mod implementations;
pub mod mcu_utils;
mod motor_layout;
mod optional_output;
mod servo;
mod spi_port;
mod uart_port;
mod voltage_reader;

pub use implementations::{
    dshot_delays, get_spawners, make_hardware, BatteryMeter, Irqs, SERVO_TIMER, USB_DEVICE_PRODUCT,
    USB_DM, USB_DP, USB_PERIPHERAL,
};

pub use optional_output::OptionalOutput;
pub use spi_port::SpiMaker;
pub use uart_port::UartMaker;

pub use config_storage::{ConfigStorageError, ConfigStore};

pub use servo::ServoDriver;

use crate::hal::motor_layout::MotorLayout;

#[allow(dead_code)]
pub struct FcHardware<
    UsbDp: DpPin<USB_PERIPHERAL>,
    UsbDm: DmPin<USB_PERIPHERAL>,
    ImuSpiMaker: SpiMaker,
    RadioUartMaker: UartMaker,
    MspUartMaker: UartMaker,
    GpsUartMaker: UartMaker,
    ConfigStoreType: ConfigStore,
> {
    pub blue_pin: Peri<'static, AnyPin>,
    pub green_pin: Peri<'static, AnyPin>,
    pub yellow_pin: Peri<'static, AnyPin>,
    pub usb_peripheral: Peri<'static, USB_PERIPHERAL>,
    pub usb_dp: Peri<'static, UsbDp>,
    pub usb_dm: Peri<'static, UsbDm>,

    pub battery_meter: BatteryMeter,

    pub imu_spi: ImuSpiMaker,

    pub motor_layout: MotorLayout,

    pub radio_uart: RadioUartMaker,

    pub vtx_power_toggle: OptionalOutput,

    pub msp_uart: Option<MspUartMaker>,
    pub gps_uart: Option<GpsUartMaker>,

    pub config_store: ConfigStoreType,
}

// Used to indicate a generic type of the make_hardware methods until we have
// generic type aliases
// https://github.com/rust-lang/rust/issues/63063
#[macro_export]
macro_rules! generic_hardware_type {
    () => {
        FcHardware<
            USB_DP,
            USB_DM,
            impl SpiMaker,
            impl UartMaker,
            impl UartMaker,
            impl UartMaker,
            impl ConfigStore,
        >
    };
}

pub struct Spawners {
    pub spawner_high: SendSpawner,
    pub spawner_low: SendSpawner,
}
