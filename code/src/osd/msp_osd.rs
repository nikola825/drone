// We do a lot of byte array manipulation here. Forbid indexing on things of uknown size
#![forbid(clippy::indexing_slicing)]

use crate::{
    hal::OptionalOutput,
    msp::{transmit_fc_variant, transmit_status, transmit_sticks},
    msp_displayport::{
        clear_display, draw_display, set_resolution, write_string_to_screen, HDZeroResolution,
    },
    navigation_utils::HeadingOffset,
    osd::char_map_hdzero_inav::OSDSymbol,
};
use embassy_executor::SendSpawner;
use num_traits::float::FloatCore;

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

pub trait IntegerStringPrinting {
    fn print_integer_value(&mut self, value: impl Into<u16>);
    fn print_integer_value_keep_leading_zeros(&mut self, value: impl Into<u16>);
}

impl IntegerStringPrinting for [u8] {
    fn print_integer_value(&mut self, value: impl Into<u16>) {
        let mut value: u16 = value.into();
        let mut rightmost_digit = true;

        for character in self.iter_mut().rev() {
            if !rightmost_digit && value == 0 {
                *character = OSDSymbol::Blank.into();
            } else {
                *character = b'0' + (value % 10) as u8;
                value /= 10;
                rightmost_digit = false;
            }
        }
    }

    fn print_integer_value_keep_leading_zeros(&mut self, value: impl Into<u16>) {
        let mut value: u16 = value.into();

        for character in self.iter_mut().rev() {
            *character = b'0' + (value % 10) as u8;

            value /= 10;
        }
    }
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
    let value = value.clamp(0, 9999) as u16;

    returned_string[4..6].print_integer_value_keep_leading_zeros(value);
    returned_string[3] = b'.';
    returned_string[1..3].print_integer_value(value / 100);

    returned_string
}

fn uint8_to_byte_string(
    value: u8,
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

    returned_string[1..4].print_integer_value(value);

    returned_string
}

fn uint16_to_byte_string(value: u16, symbol_character: OSDSymbol) -> [u8; 5] {
    let mut returned_string: [u8; 5] = [symbol_character.into(), b'_', b'_', b'_', b'_'];

    returned_string[1..5].print_integer_value(value);

    returned_string
}

fn heading_offset_to_string(offset: HeadingOffset, symbol_character: OSDSymbol) -> [u8; 6] {
    let (offset, left_symbol, right_symbol) = match offset {
        HeadingOffset::CounterClockwise(offset) => (offset, OSDSymbol::ArrowLeft, OSDSymbol::Blank),
        HeadingOffset::Clockwise(offset) => (offset, OSDSymbol::Blank, OSDSymbol::ArrowRight),
    };

    let mut returned_string: [u8; 6] = [
        symbol_character.into(),
        left_symbol.into(),
        b'_',
        b'_',
        b'_',
        right_symbol.into(),
    ];

    returned_string[2..5].print_integer_value(offset);

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

    let battery_information = shared_state.get_battery_information().await;
    let link_stats = shared_state.get_link_state().await;
    let rssi = link_stats.best_rssi();
    let link_quality = link_stats.link_quality;
    let throttle_percentage = command_state.commands.throttle_percent();
    let arming_message = command_state.arming_tracker.arming_message();

    let mut left_column = ColumnPositionTracker::new(LEFT_COLUMN, 2);
    let mut right_column = ColumnPositionTracker::new(RIGHT_COLUMN, 2);

    let bat_string = float_to_byte_string(
        battery_information.get_total_voltage(),
        OSDSymbol::Battery,
        OSDSymbol::Vols,
    );
    let cell_string = float_to_byte_string(
        battery_information.get_cell_voltage(),
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

                let home_heading_offset = packet.motion_heading.offset_to(&home_heading);
                let home_heading_offset_string =
                    heading_offset_to_string(home_heading_offset, OSDSymbol::Home);
                add_string_to_column(tx, &mut right_column, home_heading_offset_string).await?;

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
