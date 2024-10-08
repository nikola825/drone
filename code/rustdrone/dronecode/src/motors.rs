use embassy_stm32::{gpio::{AnyPin, Level, Output, Pin}, pac::GPIO, Peripheral};

use crate::dshot::dshot_send;

pub struct Motor {
    port: u8,
    pin: u8,
    output: Output<'static>
}

impl Motor {
    pub fn new(pin: impl Pin + 'static)->Self {
        let port = pin.port();
        let pin_number = pin.pin();
        let output = Output::new(pin, Level::Low, embassy_stm32::gpio::Speed::VeryHigh);
        Motor {
            port:port,
            pin:pin_number,
            output: output
        }
    }

    pub fn send_value(&self, value: u16) {
        dshot_send(GPIO(self.port as _).bsrr(), self.pin as _, value);
    }
}