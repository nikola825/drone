use cortex_m::peripheral::SCB;
use embassy_stm32::{gpio::Pin, Peri};

pub fn get_pin_gpio<T: Pin>(pin: &Peri<'static, T>) -> embassy_stm32::pac::gpio::Gpio {
    {
        unsafe {
            {
                embassy_stm32::pac::gpio::Gpio::from_ptr(
                    (1476526080usize + 1024usize * (pin.port() as usize)) as _,
                )
            }
        }
    }
}

pub fn reset_fc() -> ! {
    SCB::sys_reset();
}
