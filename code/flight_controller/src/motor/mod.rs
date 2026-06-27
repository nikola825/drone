use embassy_stm32::{
    gpio::{AnyPin, Flex, Level, Pin},
    pac::gpio::Gpio,
    Peri,
};

use crate::hal::mcu_utils::get_pin_gpio;

pub mod esc_dshot;
pub mod esc_serial;

pub struct Motor {
    port: Gpio,
    pin: u8,
    flex: Flex<'static>,
}

impl Motor {
    pub fn new(pin: Peri<'static, AnyPin>) -> Self {
        let port = get_pin_gpio(&pin);
        let pin_number = pin.pin();

        let flex = Flex::new(pin);

        let mut motor = Motor {
            port,
            pin: pin_number,
            flex,
        };

        motor.enter_dshot_mode();

        motor
    }

    pub fn enter_dshot_mode(&mut self) {
        self.flex
            .set_as_output(embassy_stm32::gpio::Speed::VeryHigh);
        self.flex.set_level(Level::Low);
    }

    pub fn enter_serial_mode(&mut self) {
        self.flex.set_as_input_output_pull(
            embassy_stm32::gpio::Speed::VeryHigh,
            embassy_stm32::gpio::Pull::Up,
        );
        self.flex.set_high();
    }
}
