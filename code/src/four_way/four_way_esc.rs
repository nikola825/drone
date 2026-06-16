use core::mem::offset_of;

use zerocopy::{big_endian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    four_way::esc_control::{device_init_flash, device_read, device_reset, device_write},
    hal::{
        Disconnected, EscCommunicationError, EscSoftSerial, Leds, PacketHeaderType,
        UsbSerialWrapper,
    },
};

fn calculate_crc(data: &[u8]) -> u16 {
    fn crc_byte(mut crc: u16, data: u8) -> u16 {
        crc ^= (data as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1
            }
        }

        crc
    }
    let mut crc = 0;

    for x in data {
        crc = crc_byte(crc, *x);
    }

    crc
}

enum FourWayError {
    UsbDisconnected(Disconnected),
    ExecutionError(FourWayExecutionError),
}

enum FourWayExecutionError {
    #[allow(dead_code)]
    EscCommunicationError(EscCommunicationError),
    ParseError,
    #[allow(dead_code)]
    BadCrc(u16, u16, FourWayHeader),
    #[allow(dead_code)]
    EscNotSelected,
    #[allow(dead_code)]
    BadEscSelected,
}

impl From<Disconnected> for FourWayError {
    fn from(value: Disconnected) -> Self {
        FourWayError::UsbDisconnected(value)
    }
}

impl From<FourWayExecutionError> for FourWayError {
    fn from(value: FourWayExecutionError) -> Self {
        FourWayError::ExecutionError(value)
    }
}

impl From<EscCommunicationError> for FourWayExecutionError {
    fn from(value: EscCommunicationError) -> Self {
        FourWayExecutionError::EscCommunicationError(value)
    }
}

impl From<EscCommunicationError> for FourWayError {
    fn from(value: EscCommunicationError) -> Self {
        FourWayError::ExecutionError(FourWayExecutionError::from(value))
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes, PartialEq)]
#[repr(u8)]
pub enum FourWayCommand {
    cmd_Remote_Escape = 0x2e,
    cmd_Local_Escape = 0x2f,
    #[allow(dead_code)]
    cmd_IntefaceExit = 0x34,
    #[allow(dead_code)]
    cmd_DeviceReset = 0x35,
    #[allow(dead_code)]
    cmd_DeviceInitFlash = 0x37,
    #[allow(dead_code)]
    cmd_DeviceRead = 0x3a,
    #[allow(dead_code)]
    cmd_DeviceWrite = 0x3b,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(u8)]
enum AckStatus {
    ACK_OK = 0x00,
    ACK_I_INVALID_CMD = 0x02,
    ACK_I_INVALID_CRC = 0x03,
    // ACK_I_VERIFY_ERROR = 0x04,
    // ACK_I_INVALID_CHANNEL = 0x08,
    ACK_I_INVALID_PARAM = 0x09,
    ACK_D_GENERAL_ERROR = 0x0f,
}

#[derive(TryFromBytes, Immutable, KnownLayout, Unaligned, IntoBytes)]
#[repr(C)]
struct FourWayHeader {
    init_command: FourWayCommand,
    command: FourWayCommand,
    address: big_endian::U16,
    param_len: u8,
}

impl PacketHeaderType for FourWayHeader {
    fn try_extract_header(buffer: &[u8]) -> Option<Self> {
        if buffer.len() >= size_of::<Self>() {
            if let Ok((header, _)) = Self::try_read_from_prefix(buffer) {
                if header.init_command == FourWayCommand::cmd_Local_Escape {
                    return Some(header);
                }
            }
        }

        None
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct FourWayRequest {
    header: FourWayHeader,
    payload: [u8; 256],
    crc: big_endian::U16,
}

impl FourWayRequest {
    fn try_from_bytes(
        header: FourWayHeader,
        buffer: &[u8; 280],
    ) -> Result<Self, FourWayExecutionError> {
        let payload_len: usize = if header.param_len == 0 {
            256
        } else {
            header.param_len as usize
        };

        let payload = &buffer[offset_of!(Self, payload)..offset_of!(Self, payload) + payload_len];
        let crc_offset = offset_of!(Self, crc) - 256 + payload.len();

        let calculated_crc = calculate_crc(&buffer[0..crc_offset]);
        let received_crc: u16 = big_endian::U16::try_read_from_bytes(
            &buffer[crc_offset..crc_offset + size_of::<big_endian::U16>()],
        )
        .ok()
        .ok_or(FourWayExecutionError::ParseError)?
        .into();

        if calculated_crc != received_crc {
            return Err(FourWayExecutionError::BadCrc(
                received_crc,
                calculated_crc,
                header,
            ));
        }

        let payload_offset = offset_of!(Self, payload);

        let mut result = Self {
            header,
            payload: [0u8; 256],
            crc: calculated_crc.into(),
        };

        result.payload[0..payload_len]
            .copy_from_slice(&buffer[payload_offset..payload_offset + payload_len]);

        Ok(result)
    }
}

pub trait FourWayResponsePayload: IntoBytes + Immutable + KnownLayout + Unaligned {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct FourWayResponse<Payload: FourWayResponsePayload> {
    header: FourWayHeader,
    payload: Payload,
    ack: AckStatus,
    crc: big_endian::U16,
}

impl<Payload: FourWayResponsePayload> FourWayResponse<Payload> {
    fn new(command: FourWayCommand, payload: Payload, address: u16, ack: AckStatus) -> Self {
        let mut message = FourWayResponse {
            header: FourWayHeader {
                init_command: FourWayCommand::cmd_Remote_Escape,
                command,
                address: address.into(),
                param_len: size_of::<Payload>() as u8,
            },
            payload,
            ack,
            crc: 0.into(),
        };

        let crc_part_end = offset_of!(FourWayResponse<Payload>, crc);

        let crc = calculate_crc(&message.as_bytes()[0..crc_part_end]);

        message.crc = crc.into();

        message
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct FourWayErrorPayload {}

impl FourWayResponsePayload for FourWayErrorPayload {}
impl FourWayResponsePayload for [u8; 256] {}

impl FourWayResponse<[u8; 256]> {
    fn new_with_byte_array(
        len: u8,
        payload: [u8; 256],
        command: FourWayCommand,
        address: u16,
        ack: AckStatus,
    ) -> Self {
        let mut message = FourWayResponse {
            header: FourWayHeader {
                init_command: FourWayCommand::cmd_Remote_Escape,
                command,
                address: address.into(),
                param_len: len,
            },
            payload,
            ack,
            crc: 0.into(),
        };

        let (len, bytes) = message.make_bytes();
        let crc = calculate_crc(&bytes[0..len - 2]);

        message.crc = crc.into();

        message
    }

    pub fn make_bytes(&self) -> (usize, [u8; 280]) {
        let mut buffer = [0u8; 280];
        let part_1_len = offset_of!(Self, payload) + (self.header.param_len as usize);
        let bytes = self.as_bytes();

        let part_2_offset = offset_of!(Self, ack);
        let part_2_len = size_of::<Self>() - part_2_offset;

        buffer[..part_1_len].copy_from_slice(&bytes[..part_1_len]);

        buffer[part_1_len..(part_1_len + part_2_len)]
            .copy_from_slice(&bytes[part_2_offset..(part_2_offset + part_2_len)]);

        (part_1_len + part_2_len, buffer)
    }
}

type FourWayErrorResponse = FourWayResponse<FourWayErrorPayload>;

impl FourWayErrorResponse {
    pub fn bad_command(command: FourWayCommand) -> Self {
        FourWayResponse::new(
            command,
            FourWayErrorPayload {},
            0,
            AckStatus::ACK_I_INVALID_CMD,
        )
    }

    pub fn bad_crc() -> Self {
        FourWayResponse::new(
            FourWayCommand::cmd_Local_Escape,
            FourWayErrorPayload {},
            0,
            AckStatus::ACK_I_INVALID_CRC,
        )
    }

    pub fn bad_parse() -> Self {
        FourWayResponse::new(
            FourWayCommand::cmd_Local_Escape,
            FourWayErrorPayload {},
            0,
            AckStatus::ACK_I_INVALID_PARAM,
        )
    }

    pub fn general_fail() -> Self {
        FourWayResponse::new(
            FourWayCommand::cmd_Local_Escape,
            FourWayErrorPayload {},
            0,
            AckStatus::ACK_D_GENERAL_ERROR,
        )
    }
}

impl From<FourWayExecutionError> for FourWayErrorResponse {
    fn from(value: FourWayExecutionError) -> Self {
        match value {
            FourWayExecutionError::ParseError => FourWayErrorResponse::bad_parse(),
            FourWayExecutionError::BadCrc(_, _, _) => FourWayErrorResponse::bad_crc(),
            FourWayExecutionError::EscCommunicationError(_) => FourWayErrorResponse::general_fail(),
            FourWayExecutionError::EscNotSelected => FourWayErrorResponse::general_fail(),
            FourWayExecutionError::BadEscSelected => FourWayErrorResponse::general_fail(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct InterfaceExitResponse {
    dummy: u8,
}

impl FourWayResponsePayload for InterfaceExitResponse {}

struct FourWayLoopContext {
    selected_esc: Option<usize>,
    continue_loop: bool,
}

impl FourWayLoopContext {
    pub fn end_loop(&mut self) {
        self.continue_loop = false;
    }

    pub fn pick_esc<'a, const ESC_COUNT: usize>(
        &self,
        escs: &'a mut [EscSoftSerial; ESC_COUNT],
    ) -> Result<&'a mut EscSoftSerial, FourWayExecutionError> {
        let selected_esc = self
            .selected_esc
            .ok_or(FourWayExecutionError::EscNotSelected)?;

        if selected_esc < ESC_COUNT {
            Ok(&mut escs[selected_esc])
        } else {
            Err(FourWayExecutionError::BadEscSelected)
        }
    }
}

pub async fn four_way_loop<const ESC_COUNT: usize>(
    usb: &mut UsbSerialWrapper,
    escs: &mut [EscSoftSerial; ESC_COUNT],
    leds: &mut Leds,
) -> Result<(), Disconnected> {
    unsafe {
        let mut peripherals = cortex_m::Peripherals::steal();
        peripherals.SCB.disable_icache();
    }

    let mut context = FourWayLoopContext {
        continue_loop: true,
        selected_esc: None,
    };

    while context.continue_loop {
        let header: FourWayHeader = usb.advance_until_header().await?;

        handle_4way_passthrough_message(usb, escs, header, &mut context, leds).await?;
    }

    Ok(())
}

async fn handle_4way_passthrough_message<const ESC_COUNT: usize>(
    usb: &mut UsbSerialWrapper,
    escs: &mut [EscSoftSerial; ESC_COUNT],
    header: FourWayHeader,
    context: &mut FourWayLoopContext,
    leds: &mut Leds,
) -> Result<(), Disconnected> {
    let mut buffer: [u8; 280] = [0u8; 280];
    usb.read_exact(&mut buffer[0..size_of::<FourWayHeader>()])
        .await?;

    let actual_param_len: usize = if header.param_len == 0 {
        256
    } else {
        header.param_len as usize
    };

    let remainder_len =
        size_of::<FourWayRequest>() - 256 + actual_param_len - size_of::<FourWayHeader>();
    usb.read_exact(
        &mut buffer[size_of::<FourWayHeader>()..size_of::<FourWayHeader>() + remainder_len],
    )
    .await?;

    let received_command = FourWayRequest::try_from_bytes(header, &buffer);

    let result = if let Ok(command) = received_command {
        execute_command(escs, usb, &command, context, leds).await
    } else if let Err(error) = received_command {
        Err(FourWayError::from(error))
    } else {
        Ok(())
    };

    if let Err(error) = result {
        match error {
            FourWayError::UsbDisconnected(disconnected) => {
                return Err(disconnected);
            }
            FourWayError::ExecutionError(four_way_execution_error) => {
                usb.write_chunked(FourWayErrorResponse::from(four_way_execution_error).as_bytes())
                    .await?;
            }
        }
    }

    Ok(())
}

async fn execute_command<const ESC_COUNT: usize>(
    escs: &mut [EscSoftSerial; ESC_COUNT],
    usb: &mut UsbSerialWrapper,
    command: &FourWayRequest,
    context: &mut FourWayLoopContext,
    leds: &mut Leds,
) -> Result<(), FourWayError> {
    match command.header.command {
        FourWayCommand::cmd_DeviceInitFlash => {
            if command.header.param_len > 0 {
                let selected_esc = command.payload[0] as usize;
                if selected_esc > ESC_COUNT {
                    return Err(FourWayExecutionError::BadEscSelected.into());
                }
                context.selected_esc = Some(selected_esc);
            }
            let response = device_init_flash(context.pick_esc(escs)?).await?;
            send_response(usb, response, &command.header).await?;
            leds.yellow_on();
        }
        FourWayCommand::cmd_DeviceRead => {
            let len = command.payload[0];
            let (len, payload) =
                device_read(context.pick_esc(escs)?, command.header.address.into(), len)?;

            let response = FourWayResponse::new_with_byte_array(
                len,
                payload,
                command.header.command,
                command.header.address.into(),
                AckStatus::ACK_OK,
            );
            let (len, bytes) = response.make_bytes();
            usb.write_chunked(&bytes[0..len]).await?;
        }
        FourWayCommand::cmd_DeviceWrite => {
            let actual_len: usize = if command.header.param_len == 0 {
                256usize
            } else {
                command.header.param_len as usize
            };

            let response = device_write(
                context.pick_esc(escs)?,
                command.header.address.into(),
                actual_len,
                &command.payload[..actual_len],
            )
            .await?;

            send_response(usb, response, &command.header).await?;
        }
        FourWayCommand::cmd_DeviceReset => {
            let response = device_reset(context.pick_esc(escs)?).await?;
            send_response(usb, response, &command.header).await?;
        }
        FourWayCommand::cmd_IntefaceExit => {
            let response = InterfaceExitResponse { dummy: 0 };

            send_response(usb, response, &command.header).await?;

            context.end_loop();
            return Ok(());
        }
        _ => {
            usb.write_chunked(FourWayResponse::bad_command(command.header.command).as_bytes())
                .await?;
        }
    }

    Ok(())
}

async fn send_response<Payload: FourWayResponsePayload>(
    usb: &mut UsbSerialWrapper,
    payload: Payload,
    header: &FourWayHeader,
) -> Result<(), Disconnected> {
    usb.write_chunked(
        FourWayResponse::new(
            header.command,
            payload,
            header.address.into(),
            AckStatus::ACK_OK,
        )
        .as_bytes(),
    )
    .await?;

    Ok(())
}
