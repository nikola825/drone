use core::mem::offset_of;

use embassy_stm32::{mode::Async, usart::UartTx};
use zerocopy::{little_endian, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::shared_state::CommandState;

const FC_VARIANT: &[u8; 4] = b"INAV";

pub trait MSPMessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned {
    fn message_type() -> MSPMessageType;
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
pub enum MSPMessageType {
    FC_VARIANT = 0x02,  // 2 - FC variant string
    STATUS = 0x65,      // 101 - FC status - arming state mainly
    RC = 0x69,          // 105 - Stick positions
    DISPLAYPORT = 0xb6, // 182 - Display commands
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct MSPMessage<Payload: MSPMessagePayload> {
    preamble: [u8; 2],
    direction: u8,
    len: u8,
    message_type: MSPMessageType,
    payload: Payload,
    xor: u8,
}

impl<Payload: MSPMessagePayload> From<Payload> for MSPMessage<Payload> {
    fn from(payload: Payload) -> Self {
        let mut message = MSPMessage {
            preamble: *b"$M",
            direction: b'>',
            len: (size_of::<Payload>() as u8),
            message_type: Payload::message_type(),
            payload,
            xor: 0u8,
        };

        let xorred_part_start = offset_of!(MSPMessage<Payload>, len);

        message.xor = message.as_bytes()[xorred_part_start..]
            .iter()
            .fold(0u8, |accumulator, element| accumulator ^ *element);

        message
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct FcVariantMessage<const STRING_LEN: usize> {
    variant: [u8; STRING_LEN],
}

impl<const STRING_LEN: usize> MSPMessagePayload for FcVariantMessage<STRING_LEN> {
    fn message_type() -> MSPMessageType {
        MSPMessageType::FC_VARIANT
    }
}

impl<const STRING_LEN: usize> FcVariantMessage<STRING_LEN> {
    fn new(variant: &[u8; STRING_LEN]) -> Self {
        Self { variant: *variant }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct FcStatusMessage {
    unused_data_1: [u8; 6],
    armed: u8,
    unused_data_2: [u8; 15],
}

impl MSPMessagePayload for FcStatusMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::STATUS
    }
}

impl FcStatusMessage {
    fn new(armed: bool) -> Self {
        let armed = if armed { 1u8 } else { 0u8 };

        FcStatusMessage {
            armed,
            unused_data_1: Default::default(),
            unused_data_2: Default::default(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct SticksMessage {
    roll: little_endian::U16,
    pitch: little_endian::U16,
    yaw: little_endian::U16,
    throttle: little_endian::U16,
}

impl MSPMessagePayload for SticksMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::RC
    }
}

impl Default for SticksMessage {
    fn default() -> Self {
        SticksMessage {
            roll: 1500.into(),
            pitch: 1500.into(),
            yaw: 1500.into(),
            throttle: 1500.into(),
        }
    }
}

pub async fn transmit_msp_message<Payload: MSPMessagePayload>(
    tx: &mut UartTx<'static, Async>,
    message: MSPMessage<Payload>,
) -> Result<(), embassy_stm32::usart::Error> {
    tx.write(message.as_bytes()).await
}

pub async fn transmit_fc_variant(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, FcVariantMessage::new(FC_VARIANT).into()).await
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
