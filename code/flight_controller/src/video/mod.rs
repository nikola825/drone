#[cfg(feature = "msp-video")]
mod msp_video;

use embassy_executor::SendSpawner;
use embassy_time::Timer;
#[cfg(feature = "msp-video")]
use msp_video::MspVideoController as OsdControllerImplementation;
#[cfg(feature = "msp-video")]
use msp_video::OSDSymbol;

#[cfg(feature = "analog-video")]
mod analog_video;

#[cfg(feature = "analog-video")]
use analog_video::AnalogVideoController as OsdControllerImplementation;

#[cfg(feature = "analog-video")]
use analog_video::OSDSymbol;

use crate::debug_log;
use crate::gps::{GPSState, SpherePosition};
use crate::hal::{OptionalOutput, UartMaker};
use crate::logging::info;
use crate::navigation_utils::HeadingOffset;
use crate::shared_state::CommandState;
use crate::shared_state::SHARED_STATE;
use num_traits::float::FloatCore;

pub trait VideoController {
    fn new(msp_uart: impl UartMaker, vtx_power_toggle: OptionalOutput) -> Self;

    async fn init(&mut self);
    async fn enter_low_power_mode(&mut self);
    async fn enter_full_power_mode(&mut self);

    async fn transmit_boilerplate(&mut self);
    async fn transmit_sticks(&mut self, sticks: &CommandState);
    async fn transmit_arming_state(&mut self, armed: bool);
    async fn begin_refresh(&mut self);

    async fn add_string_to_left_column<const STRING_LEN: usize>(
        &mut self,
        data: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error>;

    async fn add_string_to_right_column<const STRING_LEN: usize>(
        &mut self,
        data: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error>;

    async fn end_refresh(&mut self);
}

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

async fn draw_status_osd(
    controller: &mut OsdControllerImplementation,
    command_state: &CommandState,
    gps_state: &GPSState,
    home_position: &Option<SpherePosition>,
) -> Result<(), embassy_stm32::usart::Error> {
    let battery_information = SHARED_STATE.get_battery_information();
    let link_stats = SHARED_STATE.get_link_state();
    let rssi = link_stats.best_rssi();
    let link_quality = link_stats.link_quality;
    let throttle_percentage = command_state.commands.throttle_percent();
    let arming_message = command_state.arming_tracker.arming_message();

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

    controller.begin_refresh().await;

    controller.add_string_to_left_column(bat_string).await?;
    controller.add_string_to_left_column(cell_string).await?;
    controller
        .add_string_to_left_column(throttle_string)
        .await?;

    controller
        .add_string_to_right_column(link_quality_string)
        .await?;
    controller.add_string_to_right_column(rssi_string).await?;

    if let Some(packet) = &gps_state.gps_packet {
        let sat_string = uint8_to_byte_string(
            packet.satelites_visible,
            OSDSymbol::SateliteLeft,
            OSDSymbol::Blank,
        );

        controller.add_string_to_right_column(sat_string).await?;

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

            controller.add_string_to_left_column(speed_string).await?;
            controller
                .add_string_to_left_column(altitude_string)
                .await?;
            controller.add_string_to_left_column(heading_string).await?;

            if let Some(home) = home_position {
                let home_heading = packet.position.heading_to(home);
                let home_distance_meters =
                    packet.position.distance_to_in_meters(home).clamp(0, 9999) as u16;

                let home_heading_offset = packet.motion_heading.offset_to(&home_heading);
                let home_heading_offset_string =
                    heading_offset_to_string(home_heading_offset, OSDSymbol::Home);
                controller
                    .add_string_to_right_column(home_heading_offset_string)
                    .await?;

                let home_distance_string =
                    uint16_to_byte_string(home_distance_meters, OSDSymbol::DistanceMeters);
                controller
                    .add_string_to_right_column(home_distance_string)
                    .await?;
            }
        }
    }

    controller
        .add_string_to_right_column(*arming_message)
        .await?;

    controller.end_refresh().await;

    Ok(())
}

#[embassy_executor::task]
async fn osd_refresh_task(mut controller: OsdControllerImplementation) {
    info!("OSD task start");
    let mut home: Option<SpherePosition> = None;
    controller.init().await;

    loop {
        Timer::after_millis(200).await;
        debug_log!("DISPLAY {}", counter);

        let command_state = SHARED_STATE.command_snapshot();
        let armed = command_state.arming_tracker.is_armed();
        let gps_state = SHARED_STATE.get_gps_state();

        if let Some(packet) = &gps_state.gps_packet {
            if packet.gps_data_displayable() && (!armed || home.is_none()) {
                home = Some(packet.position.clone());
            }
        }

        if armed {
            controller.enter_full_power_mode().await;
        } else if is_vtx_power_off_requested(&command_state) {
            controller.enter_low_power_mode().await;
        } else if is_vtx_power_on_requested(&command_state) {
            controller.enter_full_power_mode().await;
        }

        controller.transmit_boilerplate().await;
        controller.transmit_sticks(&command_state).await;
        controller.transmit_arming_state(armed).await;

        let _ = draw_status_osd(&mut controller, &command_state, &gps_state, &home).await;
    }
}

pub fn init_osd(uart: impl UartMaker, vtx_power_toggle: OptionalOutput, spawner: &SendSpawner) {
    let controller = OsdControllerImplementation::new(uart, vtx_power_toggle);

    spawner.must_spawn(osd_refresh_task(controller));
}
