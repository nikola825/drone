use embassy_stm32::{
    gpio::{AnyPin, Level, Output, Speed},
    Peri,
};

pub struct LedPins {
    pub blue: Peri<'static, AnyPin>,
    pub yellow: Peri<'static, AnyPin>,
    pub green: Peri<'static, AnyPin>,
}

pub struct Leds {
    blue: Output<'static>,
    yellow: Output<'static>,
    green: Output<'static>,
}

impl From<LedPins> for Leds {
    fn from(pins: LedPins) -> Self {
        Self {
            blue: Output::new(pins.blue, Level::Low, Speed::VeryHigh),
            yellow: Output::new(pins.yellow, Level::Low, Speed::VeryHigh),
            green: Output::new(pins.green, Level::Low, Speed::VeryHigh),
        }
    }
}

#[allow(dead_code)]
impl Leds {
    pub fn all_off(&mut self) {
        self.blue.set_low();
        self.green.set_low();
        self.yellow.set_low();
    }

    pub fn blue_on(&mut self) {
        self.blue.set_high();
    }

    pub fn blue_off(&mut self) {
        self.blue.set_low();
    }

    pub fn green_on(&mut self) {
        self.green.set_high();
    }

    pub fn green_off(&mut self) {
        self.green.set_low();
    }

    pub fn yellow_on(&mut self) {
        self.yellow.set_high();
    }

    pub fn yellow_off(&mut self) {
        self.yellow.set_low();
    }
}
