use embassy_stm32::{mode::Async, usart::UartTx};

use common::msp_protocol::{
    messages::{FcStatusMessage, FcVariantMessage, SticksMessage},
    protocol::transmit_msp_message,
};

use crate::shared_state::CommandState;

const FC_VARIANT_INAV: &[u8; 4] = b"INAV";

pub async fn transmit_fc_variant_inav(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, FcVariantMessage::new(FC_VARIANT_INAV).into()).await
}

pub async fn transmit_status(
    tx: &mut UartTx<'static, Async>,
    armed: bool,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, FcStatusMessage::new(armed).into()).await
}

pub async fn transmit_sticks(
    tx: &mut UartTx<'static, Async>,
    command_state: &CommandState,
) -> Result<(), embassy_stm32::usart::Error> {
    let sticks = SticksMessage {
        pitch: command_state.commands.pitch_servo().into(),
        roll: command_state.commands.roll_servo().into(),
        yaw: command_state.commands.yaw_servo().into(),
        throttle: command_state.commands.throttle_servo().into(),
    };
    transmit_msp_message(tx, sticks.into()).await
}
