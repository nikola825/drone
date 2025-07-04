use core::{
    cmp::{max, min},
    mem::offset_of,
};
use embassy_executor::SendSpawner;
use num_traits::float::FloatCore;
use zerocopy::{little_endian, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{
    gps::{GPSState, SpherePosition},
    hw_select::UartMaker,
    logging::info,
    shared_state::SharedState,
};
use embassy_stm32::{
    mode::Async,
    usart::{Parity, StopBits, UartRx, UartTx},
};
use embassy_time::Timer;

const FC_VARIANT: &[u8; 4] = b"INAV";
const CELL_COUNT: u32 = 4;

trait MSPMessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned {
    fn message_type() -> MSPMessageType;
}

trait MSPDisplayPortmessagePayload: IntoBytes + Immutable + KnownLayout + Unaligned {
    fn message_type() -> MSPDisplayportMessageType;
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(u8)]
enum MSPMessageType {
    FC_VARIANT = 0x02,  // 2 - FC variant string
    STATUS = 0x65,      // 101 - FC status - arming state mainly
    RC = 0x69,          // 105 - Stick positions
    DISPLAYPORT = 0xb6, // 182 - Display commands
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
enum HDZeroResolution {
    SD_3016 = 0x00,
    HD_5018 = 0x01,
    HD_3016 = 0x02,
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
struct MSPMessage<Payload: MSPMessagePayload> {
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

async fn transmit_msp_message<Payload: MSPMessagePayload>(
    tx: &mut UartTx<'static, Async>,
    message: MSPMessage<Payload>,
) -> Result<(), embassy_stm32::usart::Error> {
    tx.write(message.as_bytes()).await
}

async fn transmit_fc_variant(
    tx: &mut UartTx<'static, Async>,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, FcVariantMessage::new(FC_VARIANT).into()).await
}

async fn transmit_status(
    tx: &mut UartTx<'static, Async>,
    armed: bool,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, FcStatusMessage::new(armed).into()).await
}

async fn transmit_sticks(
    tx: &mut UartTx<'static, Async>,
    sticks: SticksMessage,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, sticks.into()).await
}

async fn clear_display(tx: &mut UartTx<'static, Async>) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayPortClearMessage {}.into()).await
}

async fn draw_display(tx: &mut UartTx<'static, Async>) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayPortDrawMessage {}.into()).await
}

async fn set_resolution(
    tx: &mut UartTx<'static, Async>,
    resolution: HDZeroResolution,
) -> Result<(), embassy_stm32::usart::Error> {
    transmit_msp_message(tx, DisplayResolutionMessage::new(resolution).into()).await
}

async fn write_string<const STRING_LEN: usize>(
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

fn make_msp_uart_pair(
    uart_getter: impl UartMaker,
) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let (rx, tx) = uart_getter.make_uart(115200, Parity::ParityNone, StopBits::STOP1, false);

    (rx, tx)
}

fn float_to_byte_string(value: f32, symbol_character: u8, unit_character: u8) -> [u8; 7] {
    let mut returned_string: [u8; 7] = [
        symbol_character,
        b'_',
        b'_',
        b'.',
        b'_',
        b'_',
        unit_character,
    ];

    let value = (value * 100f32).round() as i32;
    let value = min(value, 9999);
    let mut value = max(0, value);

    returned_string[5] = b'0' + (value % 10) as u8;
    value /= 10;
    returned_string[4] = b'0' + (value % 10) as u8;
    value /= 10;
    returned_string[3] = b'.';
    returned_string[2] = b'0' + (value % 10) as u8;
    value /= 10;
    returned_string[1] = b'0' + (value % 10) as u8;

    returned_string
}

fn uint8_to_byte_string(mut value: u8, symbol_character: u8, unit_character: u8) -> [u8; 5] {
    let mut returned_string: [u8; 5] = [symbol_character, b'_', b'_', b'_', unit_character];
    returned_string[3] = b'0' + (value % 10);
    value /= 10;
    returned_string[2] = b'0' + (value % 10);
    value /= 10;
    returned_string[1] = b'0' + (value % 10);

    for character in &mut returned_string[1..3] {
        if *character == b'0' {
            *character = b' ';
        } else {
            break;
        }
    }

    returned_string
}

fn uint16_to_byte_string(mut value: u16, symbol_character: u8) -> [u8; 5] {
    let mut returned_string: [u8; 5] = [symbol_character, b'_', b'_', b'_', b'_'];

    returned_string[4] = b'0' + ((value % 10) as u8);
    value /= 10;
    returned_string[3] = b'0' + ((value % 10) as u8);
    value /= 10;
    returned_string[2] = b'0' + ((value % 10) as u8);
    value /= 10;
    returned_string[1] = b'0' + ((value % 10) as u8);

    for character in &mut returned_string[1..4] {
        if *character == b'0' {
            *character = b' ';
        } else {
            break;
        }
    }

    returned_string
}

#[allow(clippy::too_many_arguments)]
async fn draw_status_osd(
    tx: &mut UartTx<'static, Async>,
    bat_voltage: f32,
    link_quality: u8,
    rssi: u8,
    arming_message: &'static [u8; 3],
    throttle_percentage: u8,
    gps_state: &GPSState,
    home_position: &Option<SpherePosition>,
) -> Result<(), embassy_stm32::usart::Error> {
    let bat_string = float_to_byte_string(bat_voltage, b'\x63', b'\x1f');
    let cell_string = float_to_byte_string(bat_voltage / (CELL_COUNT as f32), b'\x63', b'\x1f');
    let link_quality_string = uint8_to_byte_string(link_quality, b'\x02', b'\x25');
    let rssi_string = uint8_to_byte_string(rssi, b'\x01', b'\x13');
    let throttle_string = uint8_to_byte_string(throttle_percentage, b'\x95', b'\x25');

    clear_display(tx).await?;
    write_string(tx, 2, 0, bat_string).await?;
    write_string(tx, 3, 0, cell_string).await?;
    write_string(tx, 4, 0, throttle_string).await?;

    write_string(tx, 2, 44, link_quality_string).await?;
    write_string(tx, 3, 44, rssi_string).await?;
    write_string(tx, 4, 44, *arming_message).await?;

    if let Some(packet) = &gps_state.gps_packet {
        let sat_string = uint8_to_byte_string(packet.satelites_visible, b'\x08', b' ');

        write_string(tx, 5, 44, sat_string).await?;

        if packet.gps_data_displayable() {
            let speed_string =
                uint16_to_byte_string(packet.ground_speed.as_kmh_multiple(1) as u16, b'\x90');
            let altitude_string =
                uint16_to_byte_string(packet.height_mean_sea_level.as_meters() as u16, b'\x76');

            let heading_string =
                uint16_to_byte_string(packet.motion_heading.as_degrees_0_360(), b'\x0c');

            write_string(tx, 5, 0, speed_string).await?;
            write_string(tx, 6, 0, altitude_string).await?;
            write_string(tx, 6, 44, heading_string).await?;

            if let Some(home) = home_position {
                let home_heading = packet.position.heading_to(home);

                let home_heading_string =
                    uint16_to_byte_string(home_heading.as_degrees_0_360(), b'\x0a');
                write_string(tx, 7, 44, home_heading_string).await?;
            }
        }
    }

    draw_display(tx).await
}

#[embassy_executor::task]
async fn osd_refresh_task(mut tx: UartTx<'static, Async>, shared_state: &'static SharedState) {
    info!("OSD task start");
    let mut home: Option<SpherePosition> = None;

    loop {
        let link_stats = shared_state.get_link_state().await;
        let command_state = shared_state.command_snapshot().await;
        let battery_voltage = shared_state.get_voltage().await;
        let armed = command_state.arming_tracker.is_armed();
        let arming_message = command_state.arming_tracker.arming_message();
        let gps_state = shared_state.get_gps_state().await;

        if let Some(packet) = &gps_state.gps_packet {
            if !armed && packet.gps_data_displayable() {
                home = Some(packet.position.clone());
            }
        }

        let sticks = SticksMessage {
            pitch: command_state.commands.pitch_servo().into(),
            roll: command_state.commands.roll_servo().into(),
            yaw: command_state.commands.yaw_servo().into(),
            throttle: command_state.commands.throttle_servo().into(),
        };

        let _ = set_resolution(&mut tx, HDZeroResolution::HD_5018).await;
        let _ = transmit_fc_variant(&mut tx).await;
        let _ = transmit_sticks(&mut tx, sticks).await;
        let _ = transmit_status(&mut tx, armed).await;
        let _ = draw_status_osd(
            &mut tx,
            battery_voltage,
            link_stats.link_quality,
            max(link_stats.rssi1, link_stats.rssi2),
            arming_message,
            command_state.commands.throttle_percent(),
            &gps_state,
            &home,
        )
        .await;
        Timer::after_millis(200).await;
    }
}

pub fn init_msp_osd(
    msp_uart: impl UartMaker,
    spawner: &SendSpawner,
    shared_state: &'static SharedState,
) {
    let (_, msp_tx) = make_msp_uart_pair(msp_uart);

    spawner.must_spawn(osd_refresh_task(msp_tx, shared_state));
}
