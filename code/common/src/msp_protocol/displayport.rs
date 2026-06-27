use zerocopy::{Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::msp_protocol::protocol::{MSPMessage, MSPMessagePayload, MSPMessageType};

pub trait MSPDisplayPortmessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned {
    fn message_type() -> MSPDisplayportMessageType;
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum MSPDisplayportMessageType {
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
pub struct MSPDisplayPortmessage<Payload: MSPDisplayPortmessagePayload> {
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
pub struct DisplayResolutionMessage {
    unused: u8,
    resolution: HDZeroResolution,
}

impl MSPDisplayPortmessagePayload for DisplayResolutionMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::CONFIG
    }
}

impl DisplayResolutionMessage {
    pub fn new(resolution: HDZeroResolution) -> Self {
        DisplayResolutionMessage {
            unused: 0,
            resolution,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DisplayPortWriteMessage<const STRING_LEN: usize> {
    pub row: u8,
    pub col: u8,
    pub unused: u8,
    pub data: [u8; STRING_LEN],
}

impl<const STRING_LEN: usize> MSPDisplayPortmessagePayload for DisplayPortWriteMessage<STRING_LEN> {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::WRITE
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DisplayPortClearMessage {}

impl MSPDisplayPortmessagePayload for DisplayPortClearMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::CLEAR
    }
}
#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DisplayPortDrawMessage {}

impl MSPDisplayPortmessagePayload for DisplayPortDrawMessage {
    fn message_type() -> MSPDisplayportMessageType {
        MSPDisplayportMessageType::DRAW
    }
}
