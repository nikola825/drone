use core::mem::offset_of;

use embassy_executor::SendSpawner;
use embassy_futures::join::join;
use embassy_time::Timer;
use zerocopy::{little_endian, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    four_way::four_way_esc::four_way_loop,
    generic_hardware_type,
    hal::{
        Disconnected, EscSoftSerial, Leds, MotorLayout, PacketHeaderType, UsbPeripheral,
        UsbSerialWrapper,
    },
    msp::protocol::{FcVariantMessage, MSPHeader, MSPMessage, MSPMessagePayload, MSPMessageType},
};

const FC_VARIANT_BETAFLIGHT: &[u8; 4] = b"BTFL";

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct ApiVersionMessage {
    protocol_version: u8,
    api_version_major: u8,
    api_version_minor: u8,
}

impl MSPMessagePayload for ApiVersionMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_API_VERSION
    }
}

impl Default for ApiVersionMessage {
    fn default() -> Self {
        Self {
            protocol_version: 0,
            api_version_major: 1,
            api_version_minor: 48,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, Default)]
#[repr(C)]
pub struct BatteryStateMessage {
    cell_count: u8,
    battery_capacity_mah: little_endian::U16,
    battery_voltage_div_10: u8,
    mah_drawn: little_endian::U16,
    amperage_div_100: little_endian::U16,
    battery_alerts: u8,
    battery_voltage_div_100: little_endian::U16,
}

impl MSPMessagePayload for BatteryStateMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_BATTERY_STATE
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct MotorConfigMessage {
    min_throttle: little_endian::U16,
    max_throttle: little_endian::U16,
    min_command: little_endian::U16,
    motor_count: u8,
    pole_count: u8,
    use_dshot_telemetry: bool,
    esc_sensor_available: bool,
}

impl MSPMessagePayload for MotorConfigMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_MOTOR_CONFIG
    }
}

impl MotorConfigMessage {
    pub fn new(motor_count: u8) -> Self {
        MotorConfigMessage {
            min_throttle: 0.into(),
            max_throttle: 2000.into(),
            min_command: 1000.into(),
            motor_count,
            pole_count: 14,
            use_dshot_telemetry: false,
            esc_sensor_available: false,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct SetPassthroughResponse {
    motor_count: u8,
}

impl MSPMessagePayload for SetPassthroughResponse {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_SET_PASSTHROUGH
    }
}

impl SetPassthroughResponse {
    pub fn new(motor_count: u8) -> Self {
        Self { motor_count }
    }
}

pub struct ReceivedMspMessage {
    header: MSPHeader,
    _payload: [u8; 256],
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
        if header.direction != b'<' {
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
                _payload: payload,
            })
        }
    }
}

impl PacketHeaderType for MSPHeader {
    fn try_extract_header(buffer: &[u8]) -> Option<Self> {
        if buffer.len() >= size_of::<Self>() {
            if let Ok((header, _)) = Self::try_read_from_prefix(buffer) {
                if header.valid() {
                    return Some(header);
                }
            }
        }

        None
    }
}

async fn transmit_msp_message_over_usb<Payload: MSPMessagePayload>(
    message: MSPMessage<Payload>,
    usb: &mut UsbSerialWrapper,
) -> Result<(), Disconnected> {
    usb.write_chunked(message.as_bytes()).await
}

pub async fn serial_passthrough_main(
    spawner: &SendSpawner,
    hardware: generic_hardware_type!(),
) -> ! {
    let leds: Leds = hardware.led_pins.into();
    spawner.must_spawn(msp_message_processor_task(
        hardware.usb,
        hardware.motor_layout,
        leds,
    ));

    loop {
        Timer::after_secs(86400).await;
    }
}

#[embassy_executor::task]
pub async fn msp_message_processor_task(
    usb_peripheral: UsbPeripheral,
    motors: MotorLayout, //escs: [EscSoftSerial; 4],
    leds: Leds,
) {
    let mut usb_port = usb_peripheral.into_usb_port(true);

    let usb_serial = UsbSerialWrapper::new(usb_port.cdc_acm);

    let escs = motors.get_serials();

    let usb_fut = usb_port.usb_device.run();
    let msp_fut = msp_listener_loop(usb_serial, escs, leds);

    join(usb_fut, msp_fut).await;
}

async fn msp_listener_loop<const ESC_COUNT: usize>(
    mut usb_serial: UsbSerialWrapper,
    mut escs: [EscSoftSerial; ESC_COUNT],
    mut leds: Leds,
) {
    loop {
        leds.all_off();

        usb_serial.wait_for_connection().await;

        leds.blue_on();

        let _ = msp_processor_loop(&mut usb_serial, &mut escs, &mut leds).await;
    }
}

async fn msp_processor_loop<const ESC_COUNT: usize>(
    usb_serial: &mut UsbSerialWrapper,
    escs: &mut [EscSoftSerial; ESC_COUNT],
    leds: &mut Leds,
) -> Result<(), Disconnected> {
    loop {
        let header: MSPHeader = usb_serial.advance_until_header().await?;

        handle_msp_message(usb_serial, escs, header, leds).await?;
    }
}

async fn handle_msp_message<const ESC_COUNT: usize>(
    usb: &mut UsbSerialWrapper,
    esc_serial: &mut [EscSoftSerial; ESC_COUNT],
    header: MSPHeader,
    leds: &mut Leds,
) -> Result<(), Disconnected> {
    let mut buffer: [u8; 280] = [0u8; 280];
    usb.read_exact(&mut buffer[0..size_of::<MSPHeader>()])
        .await?;

    let remainder_len = (header.len + 1) as usize; // payload + xor
    usb.read_exact(&mut buffer[size_of::<MSPHeader>()..size_of::<MSPHeader>() + remainder_len])
        .await?;

    let received_message = ReceivedMspMessage::try_from_bytes(&mut buffer, header);

    match received_message {
        Ok(result) => {
            match result.header.message_type {
                MSPMessageType::MSP_API_VERSION => {
                    transmit_msp_message_over_usb(ApiVersionMessage::default().into(), usb).await?;
                }
                MSPMessageType::FC_VARIANT => {
                    transmit_msp_message_over_usb(
                        FcVariantMessage::new(FC_VARIANT_BETAFLIGHT).into(), // pretend to be Betaflight for compatibility with AM32 configurator
                        usb,
                    )
                    .await?;
                }
                MSPMessageType::MSP_BATTERY_STATE => {
                    transmit_msp_message_over_usb(BatteryStateMessage::default().into(), usb)
                        .await?;
                }
                MSPMessageType::MSP_MOTOR_CONFIG => {
                    transmit_msp_message_over_usb(
                        MotorConfigMessage::new(ESC_COUNT as u8).into(),
                        usb,
                    )
                    .await?;
                }
                MSPMessageType::MSP_SET_PASSTHROUGH => {
                    transmit_msp_message_over_usb(
                        SetPassthroughResponse::new(ESC_COUNT as u8).into(),
                        usb,
                    )
                    .await?;
                    leds.green_on();
                    four_way_loop(usb, esc_serial, leds).await?;
                }
                _ => {}
            };
        }
        Err(_) => {
            // ignore
        }
    }

    Ok(())
}
