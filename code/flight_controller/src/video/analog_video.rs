use crate::{
    hal::{OptionalOutput, UartMaker},
    video::VideoController,
};

mod char_map;

pub use char_map::OSDSymbol;
pub struct AnalogVideoController {
    vtx_power_toggle: OptionalOutput,
}

impl VideoController for AnalogVideoController {
    fn new(_: impl UartMaker, vtx_power_toggle: OptionalOutput) -> Self {
        Self { vtx_power_toggle }
    }

    async fn init(&mut self) {
        self.vtx_power_toggle.set_high();
    }

    async fn enter_low_power_mode(&mut self) {
        self.vtx_power_toggle.set_low();
    }

    async fn enter_full_power_mode(&mut self) {
        self.vtx_power_toggle.set_high();
    }

    async fn transmit_boilerplate(&mut self) {}

    async fn transmit_sticks(&mut self, _: &crate::shared_state::CommandState) {}

    async fn transmit_arming_state(&mut self, _: bool) {}

    async fn begin_refresh(&mut self) {}

    async fn add_string_to_left_column<const STRING_LEN: usize>(
        &mut self,
        _: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error> {
        Ok(())
    }

    async fn add_string_to_right_column<const STRING_LEN: usize>(
        &mut self,
        _: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error> {
        Ok(())
    }

    async fn end_refresh(&mut self) {}
}
