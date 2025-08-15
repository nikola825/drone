use embassy_stm32::gpio::{Output, Pin};

pub struct OptionalOutput {
    output: Option<Output<'static>>,
}

impl OptionalOutput {
    #[allow(dead_code)]
    pub fn new<PinType: Pin + 'static>(
        pin: PinType,
        initial_level: embassy_stm32::gpio::Level,
    ) -> Self {
        Self {
            output: Some(Output::new(
                pin,
                initial_level,
                embassy_stm32::gpio::Speed::High,
            )),
        }
    }

    #[allow(dead_code)]
    pub fn unimplemented() -> Self {
        Self { output: None }
    }

    pub fn set_high(&mut self) {
        if let Some(toggle) = self.output.as_mut() {
            toggle.set_high();
        }
    }

    #[allow(dead_code)]
    pub fn set_low(&mut self) {
        if let Some(toggle) = self.output.as_mut() {
            toggle.set_low();
        }
    }
}
