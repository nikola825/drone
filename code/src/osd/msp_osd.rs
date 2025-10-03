use crate::{hal::OptionalOutput, osd::char_map_hdzero_inav::OSDSymbol};
use core::{
    cmp::{max, min},
    mem::offset_of,
};
use embassy_executor::SendSpawner;
use num_traits::float::FloatCore;
use zerocopy::{little_endian, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{
    gps::{GPSState, SpherePosition},
    hal::UartMaker,
    logging::info,
    shared_state::{CommandState, SharedState},
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

struct ColumnPositionTracker {
    current_row: u8,
    column: u8,
}

impl ColumnPositionTracker {
    fn new(column: u8, initial_row: u8) -> Self {
        Self {
            current_row: initial_row,
            column,
        }
    }

    fn get_next_row(&mut self) -> u8 {
        let row = self.current_row;
        self.current_row += 1;
        row
    }
}

async fn add_string_to_column<const STRING_LEN: usize>(
    tx: &mut UartTx<'static, Async>,
    position_tracker: &mut ColumnPositionTracker,
    data: [u8; STRING_LEN],
) -> Result<(), embassy_stm32::usart::Error> {
    let col = position_tracker.column;
    let row = position_tracker.get_next_row();

    write_string_to_screen(tx, row, col, data).await
}

async fn write_string_to_screen<const STRING_LEN: usize>(
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

fn float_to_byte_string(
    value: f32,
    symbol_character: OSDSymbol,
    unit_character: OSDSymbol,
) -> [u8; 7] {
    let mut returned_string: [u8; 7] = [
        symbol_character.into(),
        b'_',
        b'_',
        b'.',
        b'_',
        b'_',
        unit_character.into(),
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

fn uint8_to_byte_string(
    mut value: u8,
    symbol_character: OSDSymbol,
    unit_character: OSDSymbol,
) -> [u8; 5] {
    let mut returned_string: [u8; 5] = [
        symbol_character.into(),
        b'_',
        b'_',
        b'_',
        unit_character.into(),
    ];
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

fn uint16_to_byte_string(mut value: u16, symbol_character: OSDSymbol) -> [u8; 5] {
    let mut returned_string: [u8; 5] = [symbol_character.into(), b'_', b'_', b'_', b'_'];

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

async fn draw_status_osd(
    tx: &mut UartTx<'static, Async>,
    shared_state: &SharedState,
    command_state: &CommandState,
    gps_state: &GPSState,
    home_position: &Option<SpherePosition>,
) -> Result<(), embassy_stm32::usart::Error> {
    const LEFT_COLUMN: u8 = 0;
    const RIGHT_COLUMN: u8 = 44;

    let bat_voltage = shared_state.get_voltage().await;
    let link_stats = shared_state.get_link_state().await;
    let rssi = link_stats.best_rssi();
    let link_quality = link_stats.link_quality;
    let throttle_percentage = command_state.commands.throttle_percent();
    let arming_message = command_state.arming_tracker.arming_message();

    let mut left_column = ColumnPositionTracker::new(LEFT_COLUMN, 2);
    let mut right_column = ColumnPositionTracker::new(RIGHT_COLUMN, 2);

    let bat_string = float_to_byte_string(bat_voltage, OSDSymbol::Battery, OSDSymbol::Vols);
    let cell_string = float_to_byte_string(
        bat_voltage / (CELL_COUNT as f32),
        OSDSymbol::Battery,
        OSDSymbol::Vols,
    );
    let link_quality_string =
        uint8_to_byte_string(link_quality, OSDSymbol::LinkQuality, OSDSymbol::Percent);
    let rssi_string = uint8_to_byte_string(rssi, OSDSymbol::Rssi, OSDSymbol::Dbm);
    let throttle_string = uint8_to_byte_string(
        throttle_percentage,
        OSDSymbol::ThrottlePercentage,
        OSDSymbol::Percent,
    );

    clear_display(tx).await?;
    add_string_to_column(tx, &mut left_column, bat_string).await?;
    add_string_to_column(tx, &mut left_column, cell_string).await?;
    add_string_to_column(tx, &mut left_column, throttle_string).await?;

    add_string_to_column(tx, &mut right_column, link_quality_string).await?;
    add_string_to_column(tx, &mut right_column, rssi_string).await?;

    if let Some(packet) = &gps_state.gps_packet {
        let sat_string = uint8_to_byte_string(
            packet.satelites_visible,
            OSDSymbol::SateliteLeft,
            OSDSymbol::Blank,
        );

        add_string_to_column(tx, &mut right_column, sat_string).await?;

        if packet.gps_data_displayable() {
            let speed_string = uint16_to_byte_string(
                packet.ground_speed.as_kmh_multiple(1) as u16,
                OSDSymbol::SpeedKmh,
            );
            let altitude_string = uint16_to_byte_string(
                packet.height_mean_sea_level.as_meters() as u16,
                OSDSymbol::AltitudeMeters,
            );

            let heading_string =
                uint16_to_byte_string(packet.motion_heading.as_degrees_0_360(), OSDSymbol::Heading);

            add_string_to_column(tx, &mut left_column, speed_string).await?;
            add_string_to_column(tx, &mut left_column, altitude_string).await?;
            add_string_to_column(tx, &mut right_column, heading_string).await?;

            if let Some(home) = home_position {
                let home_heading = packet.position.heading_to(home);
                let home_distance_meters =
                    packet.position.distance_to_in_meters(home).clamp(0, 9999) as u16;

                let home_heading_string =
                    uint16_to_byte_string(home_heading.as_degrees_0_360(), OSDSymbol::Home);
                add_string_to_column(tx, &mut right_column, home_heading_string).await?;

                let home_distance_string =
                    uint16_to_byte_string(home_distance_meters, OSDSymbol::DistanceMeters);
                add_string_to_column(tx, &mut right_column, home_distance_string).await?;
            }
        }
    }

    add_string_to_column(tx, &mut right_column, *arming_message).await?;

    draw_display(tx).await
}

fn is_vtx_power_off_requested(command_state: &CommandState) -> bool {
    const THRESHOLD_LOW: u16 = 1250;
    const THRESHOLD_HIGH: u16 = 1750;

    (!command_state.arming_tracker.is_armed())
        && (command_state.commands.yaw_servo() < THRESHOLD_LOW)
        && (command_state.commands.throttle_servo() < THRESHOLD_LOW)
        && (command_state.commands.pitch_servo() < THRESHOLD_LOW)
        && (command_state.commands.roll_servo() > THRESHOLD_HIGH)
}

fn is_vtx_power_on_requested(command_state: &CommandState) -> bool {
    const THRESHOLD_LOW: u16 = 1250;
    const THRESHOLD_HIGH: u16 = 1750;

    (!command_state.arming_tracker.is_armed())
        && (command_state.commands.yaw_servo() < THRESHOLD_LOW)
        && (command_state.commands.throttle_servo() > THRESHOLD_HIGH)
        && (command_state.commands.pitch_servo() > THRESHOLD_HIGH)
        && (command_state.commands.roll_servo() > THRESHOLD_HIGH)
}

#[embassy_executor::task]
async fn osd_refresh_task(
    mut tx: UartTx<'static, Async>,
    mut vtx_power_toggle: OptionalOutput,
    shared_state: &'static SharedState,
) {
    info!("OSD task start");
    let mut home: Option<SpherePosition> = None;

    loop {
        Timer::after_millis(200).await;

        let command_state = shared_state.command_snapshot().await;
        let armed = command_state.arming_tracker.is_armed();
        let gps_state = shared_state.get_gps_state().await;

        if let Some(packet) = &gps_state.gps_packet {
            if packet.gps_data_displayable() && (!armed || home.is_none()) {
                home = Some(packet.position.clone());
            }
        }

        if armed {
            vtx_power_toggle.set_high();
        } else if is_vtx_power_off_requested(&command_state) {
            vtx_power_toggle.set_low();
        } else if is_vtx_power_on_requested(&command_state) {
            vtx_power_toggle.set_high();
        }

        let _ = set_resolution(&mut tx, HDZeroResolution::HD_5018).await;
        let _ = transmit_fc_variant(&mut tx).await;
        let _ = transmit_sticks(&mut tx, &command_state).await;
        let _ = transmit_status(&mut tx, armed).await;
        let _ = draw_status_osd(&mut tx, shared_state, &command_state, &gps_state, &home).await;
    }
}

pub fn init_msp_osd(
    msp_uart: impl UartMaker,
    vtx_power_toggle: OptionalOutput,
    spawner: &SendSpawner,
    shared_state: &'static SharedState,
) {
    let (_, msp_tx) = make_msp_uart_pair(msp_uart);

    spawner.must_spawn(osd_refresh_task(msp_tx, vtx_power_toggle, shared_state));
}
