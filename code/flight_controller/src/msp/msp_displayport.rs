use common::msp_protocol::{
    displayport::{
        DisplayPortClearMessage, DisplayPortDrawMessage, DisplayPortWriteMessage,
        DisplayResolutionMessage, HDZeroResolution,
    },
    protocol::transmit_msp_message,
};
use embassy_stm32::{mode::Async, usart::UartTx};

pub async fn clear_display(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayPortClearMessage {}.into()).await
}

pub async fn draw_display(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayPortDrawMessage {}.into()).await
}

pub async fn set_resolution(
    tx: &mut UartTx<'static, Async>,
    resolution: HDZeroResolution,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayResolutionMessage::new(resolution).into()).await
}

pub async fn write_string_to_screen<const STRING_LEN: usize>(
    tx: &mut UartTx<'static, Async>,
    row: u8,
    col: u8,
    data: [u8; STRING_LEN],
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(
        tx,
        DisplayPortWriteMessage {
            row,
            col,
            unused: 0,
            data,
        }
        .into(),
    )
    .await
}
