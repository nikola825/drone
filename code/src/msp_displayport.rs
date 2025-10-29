use embassy_stm32::{mode::Async, usart::UartTx};
use zerocopy::{Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::msp::{transmit_msp_message, MSPMessage, MSPMessagePayload, MSPMessageType};

trait MSPDisplayPortmessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned {
    fn message_type() -> MSPDisplayportMessageType;
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
enum MSPDisplayportMessageType {
    CLEAR = 0x02,  // 1 - clear display
    WRITE = 0x03,  // 3 - write string to display
    DRAW = 0x04,   // 4 - draw written strings on the display
    CONFIG = 0x05, // 5 - configure display resolution
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum HDZeroResolution {
    SD_3016 = 0x00,
    HD_5018 = 0x01,
    HD_3016 = 0x02,
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct MSPDisplayPortmessage<Payload: MSPDisplayPortmessagePayload> {
    message_type: MSPDisplayportMessageType,
    payload: Payload,
}

impl<Payload: MSPDisplayPortmessagePayload> MSPMessagePayload for MSPDisplayPortmessage<Payload> {
    fn message_type() -> MSPMessageType {
        MSPMessageType::DISPLAYPORT
    }
}

impl<Payload: MSPDisplayPortmessagePayload> From<Payload> for MSPDisplayPortmessage<Payload> {
    fn from(payload: Payload) -> Self {
        MSPDisplayPortmessage {
            message_type: Payload::message_type(),
            payload,
        }
    }
}

impl<Payload: MSPDisplayPortmessagePayload> From<Payload>
    for MSPMessage<MSPDisplayPortmessage<Payload>>
{
    fn from(payload: Payload) -> Self {
        Into::<MSPDisplayPortmessage<Payload>>::into(payload).into()
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct DisplayResolutionMessage {
    unused: u8,
    resolution: HDZeroResolution,
}

impl MSPDisplayPortmessagePayload for DisplayResolutionMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::CONFIG
    }
}

impl DisplayResolutionMessage {
    fn new(resolution: HDZeroResolution) -> Self {
        DisplayResolutionMessage {
            unused: 0,
            resolution,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct DisplayPortWriteMessage<const STRING_LEN: usize> {
    row: u8,
    col: u8,
    unused: u8,
    data: [u8; STRING_LEN],
}

impl<const STRING_LEN: usize> MSPDisplayPortmessagePayload for DisplayPortWriteMessage<STRING_LEN> {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::WRITE
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct DisplayPortClearMessage {}

impl MSPDisplayPortmessagePayload for DisplayPortClearMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::CLEAR
    }
}
#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct DisplayPortDrawMessage {}

impl MSPDisplayPortmessagePayload for DisplayPortDrawMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::DRAW
    }
}

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
