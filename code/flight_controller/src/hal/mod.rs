use embassy_executor::SendSpawner;

mod config_storage;
mod implementations;
mod leds;
pub mod mcu_utils;
mod motor_layout;
mod optional_output;
mod servo;
mod spi_port;
mod uart_port;
mod usb_port;
mod voltage_reader;

pub use implementations::{
    dshot_delays, get_spawners, make_hardware, BatteryMeter, Irqs, SERVO_TIMER, USB_DEVICE_PRODUCT,
    USB_DM, USB_DP, USB_PERIPHERAL,
};

pub use config_storage::{ConfigStorageError, ConfigStore, ConfigStoreType};
pub use leds::Leds;
pub use motor_layout::{MotorLayout, ESC_COUNT};
pub use optional_output::OptionalOutput;
pub use spi_port::SpiMaker;
pub use uart_port::UartMaker;
pub use usb_port::UsbPeripheral;

pub use usb_port::{Disconnected, PacketHeaderType, UsbSerialWrapper};

#[cfg(feature = "servo-support")]
pub use servo::ServoDriver;

use crate::hal::leds::LedPins;

#[allow(dead_code)]
pub struct FcHardware<
    ImuSpiMaker: SpiMaker,
    RadioUartMaker: UartMaker,
    MspUartMaker: UartMaker,
    GpsUartMaker: UartMaker,
> {
    pub led_pins: LedPins,
    pub usb: UsbPeripheral,

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
        $crate::hal::FcHardware<
            impl $crate::hal::SpiMaker,
            impl $crate::hal::UartMaker,
            impl $crate::hal::UartMaker,
            impl $crate::hal::UartMaker,
        >
    };
}

pub struct Spawners {
    pub spawner_high: SendSpawner,
    pub spawner_low: SendSpawner,
}
