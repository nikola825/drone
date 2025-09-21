mod stm32f411;
mod stm32h723;
mod stm32h743;

#[cfg(feature = "stm32f411")]
use stm32f411 as hal_implementation;
#[cfg(feature = "stm32h723")]
use stm32h723 as hal_implementation;
#[cfg(feature = "stm32h743")]
use stm32h743 as hal_implementation;

pub use hal_implementation::{
    dshot_delay_0, dshot_delay_0_to_1, dshot_delay_remainder, get_spawners, make_hardware,
    BatteryMeter, Irqs, USB_DEVICE_PRODUCT, USB_DM, USB_DP, USB_PERIPHERAL,
};
