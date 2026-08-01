use embassy_time::Timer;
use zerocopy::{big_endian, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{
    four_way::four_way_esc::FourWayResponsePayload,
    motor::{esc_serial::EscCommunicationError, Motor},
};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
enum EscCommand {
    CMD_PROG_FLASH = 0x01,
    CMD_READ_FLASH_SIL = 0x03,
    CMD_SET_BUFFER = 0xfe,
    CMD_SET_ADDRESS = 0xff,
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DeviceInitFlashResponse {
    signature2: u8,
    signature1: u8,
    c: u8,
    boot_pages: u8,
}

impl FourWayResponsePayload for DeviceInitFlashResponse {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DeviceWriteResponse {
    dummy: u8,
}

impl FourWayResponsePayload for DeviceWriteResponse {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct DeviceResetResponse {
    dummy: u8,
}

impl FourWayResponsePayload for DeviceResetResponse {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct EscSetAddressCommand {
    command: EscCommand,
    dummy: u8,
    address: big_endian::U16,
}

impl EscSetAddressCommand {
    pub fn new(address: u16) -> Self {
        Self {
            command: EscCommand::CMD_SET_ADDRESS,
            dummy: 0,
            address: address.into(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct EscReadCommand {
    command: EscCommand,
    length: u8,
}
impl EscReadCommand {
    pub fn new(length: u8) -> Self {
        Self {
            command: EscCommand::CMD_READ_FLASH_SIL,
            length,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct EscSetBufferCommand {
    command: EscCommand,
    dummy0: u8,
    length: big_endian::U16,
}

impl EscSetBufferCommand {
    pub fn new(length: u16) -> Self {
        Self {
            command: EscCommand::CMD_SET_BUFFER,
            dummy0: 0,
            length: length.into(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct EscProgFlashCommand {
    command: EscCommand,
    dummy0: u8,
}

impl Default for EscProgFlashCommand {
    fn default() -> Self {
        Self {
            command: EscCommand::CMD_PROG_FLASH,
            dummy0: 1,
        }
    }
}

pub async fn device_init_flash(
    esc_serial: &mut Motor,
) -> Result<DeviceInitFlashResponse, EscCommunicationError> {
    const BOOT_MSG: [u8; 17] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0x0D, b'B', b'L', b'H', b'e', b'l', b'i', 0xF4, 0x7D,
    ];

    let mut retries_remaining = 3;
    loop {
        esc_serial.send(&BOOT_MSG, false);

        let mut response = [0; 16];
        let receive_result = esc_serial.read(&mut response, 8, false, 10);
        match receive_result {
            Ok(_) => {
                return Ok(DeviceInitFlashResponse {
                    signature2: response[5],
                    signature1: response[4],
                    c: response[3],
                    boot_pages: 4,
                });
            }
            Err(error) => {
                retries_remaining -= 1;

                if retries_remaining > 0 {
                    Timer::after_millis(10).await;
                } else {
                    return Err(error);
                }
            }
        }
    }
}

pub fn device_read(
    esc_serial: &mut Motor,
    address: u16,
    length: u8,
) -> Result<(u8, [u8; 256]), EscCommunicationError> {
    let set_address_command = EscSetAddressCommand::new(address);

    esc_serial.send(set_address_command.as_bytes(), true);

    let mut set_addr_response = [0u8; 16];
    esc_serial.read(&mut set_addr_response, 0, false, 10)?;

    let read_command = EscReadCommand::new(length);

    esc_serial.send(read_command.as_bytes(), true);

    let mut recv_buffer = [0u8; 280];

    esc_serial.read(&mut recv_buffer, length as usize, true, 200)?;

    let mut result_buffer = [0u8; 256];

    result_buffer[..(length as usize)].copy_from_slice(&recv_buffer[..(length as usize)]);

    Ok((length, result_buffer))
}

pub async fn device_write(
    esc_serial: &mut Motor,
    address: u16,
    length: usize,
    payload: &[u8],
) -> Result<DeviceWriteResponse, EscCommunicationError> {
    let set_address_command = EscSetAddressCommand::new(address);

    esc_serial.send(set_address_command.as_bytes(), true);

    let mut set_addr_response = [0u8; 16];
    esc_serial.read(&mut set_addr_response, 0, false, 4)?;

    let set_buffer_command = EscSetBufferCommand::new(length as u16);

    esc_serial.send(set_buffer_command.as_bytes(), true);

    Timer::after_millis(4).await;

    esc_serial.send(&payload[..length], true);

    let mut write_response = set_addr_response;
    esc_serial.read(&mut write_response, 0, false, 80)?;

    let prog_flash_command = EscProgFlashCommand::default();

    esc_serial.send(prog_flash_command.as_bytes(), true);

    let mut prog_response = write_response;
    esc_serial.read(&mut prog_response, 0, false, 500)?;

    Ok(DeviceWriteResponse { dummy: payload[0] })
}

pub async fn device_reset(
    esc_serial: &mut Motor,
) -> Result<DeviceResetResponse, EscCommunicationError> {
    let restart_message = [0u8, 0u8];
    esc_serial.send(&restart_message, true);
    Timer::after_millis(50).await;
    Ok(DeviceResetResponse { dummy: 0 })
}
