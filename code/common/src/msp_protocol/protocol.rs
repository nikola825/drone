use core::mem::offset_of;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

pub trait MSPMessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned + Sized {
    fn message_type() -> MSPMessageType;
}

pub trait MSPReceivedPayload: MSPMessagePayload + TryFromBytes {}

#[allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(u8)]
pub enum MSPMessageType {
    MSP_API_VERSION = 0x01, // 1 - API version
    FC_VARIANT = 0x02,      // 2 - FC variant string
    MSP_REBOOT = 0x44,
    STATUS = 0x65,              // 101 - FC status - arming state mainly
    RC = 0x69,                  // 105 - Stick positions
    MSP_BATTERY_STATE = 0x82,   // 130 - Battery state
    MSP_MOTOR_CONFIG = 0x83,    // 131 - Motor config
    DISPLAYPORT = 0xb6,         // 182 - Display commands
    MSP_SET_PASSTHROUGH = 0xf5, // 245 - Enable passthrough
    MSP_CUSTOM_CONFIGURATOR = 0xe7,
    // UNKNOWN = 0xff,             // 255 - Never used
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct MSPHeader {
    pub preamble: [u8; 2],
    pub direction: u8,
    pub len: u8,
    pub message_type: MSPMessageType,
}

impl MSPHeader {
    #[allow(dead_code)]
    pub fn valid(&self) -> bool {
        self.preamble[0] == b'$'
            && self.preamble[1] == b'M'
            && (self.direction == b'>' || self.direction == b'<')
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct MSPMessage<Payload: MSPMessagePayload> {
    pub header: MSPHeader,
    pub payload: Payload,
    xor: u8,
}

impl<Payload: MSPMessagePayload> From<Payload> for MSPMessage<Payload> {
    fn from(payload: Payload) -> Self {
        let mut message = MSPMessage {
            header: MSPHeader {
                preamble: *b"$M",
                direction: b'>',
                len: (size_of::<Payload>() as u8),
                message_type: Payload::message_type(),
            },
            payload,
            xor: 0u8,
        };

        let xorred_part_start = offset_of!(MSPHeader, len);

        message.xor = message.as_bytes()[xorred_part_start..]
            .iter()
            .fold(0u8, |accumulator, element| accumulator ^ *element);

        message
    }
}

pub struct ReceivedMspMessage {
    pub header: MSPHeader,
    pub payload: [u8; 256],
}

#[allow(dead_code)]
pub enum MSPReceiveError {
    BadDirection(u8),
    BaddXor(u8, u8),
}

impl ReceivedMspMessage {
    pub fn try_from_bytes(
        buffer: &mut [u8; 280],
        header: MSPHeader,
    ) -> Result<ReceivedMspMessage, MSPReceiveError> {
        if header.direction != b'<' && header.direction != b'>' {
            Err(MSPReceiveError::BadDirection(header.direction))
        } else {
            let payload_length = header.len as usize;
            let offset_of_xor = size_of::<MSPHeader>() + payload_length;

            let xorred_part_start = offset_of!(MSPHeader, len);
            let xorred_part_end = offset_of_xor;

            let calculated_xor = buffer[xorred_part_start..xorred_part_end]
                .iter()
                .fold(0u8, |accumulator, element| accumulator ^ *element);

            let expected_xor = buffer[offset_of_xor];
            if calculated_xor != expected_xor {
                return Err(MSPReceiveError::BaddXor(calculated_xor, expected_xor));
            }

            let mut payload = [0u8; 256];
            let payload_part = &buffer[size_of::<MSPHeader>()..];
            payload[0..payload_length].copy_from_slice(&payload_part[0..payload_length]);

            Ok(ReceivedMspMessage {
                header,
                payload: payload,
            })
        }
    }
}

pub async fn transmit_msp_message<
    Payload: MSPMessagePayload,
    Transmitter: embedded_io_async::Write,
>(
    tx: &mut Transmitter,
    message: MSPMessage<Payload>,
) -> Result<(), Transmitter::Error> {
    tx.write_all(message.as_bytes()).await
}
