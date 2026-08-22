#![no_std]
#![no_main]

use crate::{
    configurator::configurator_loop::configurator_loop, four_way::four_way_esc::FourWayParameters,
    hal::make_hardware,
};
use cortex_m_rt::entry;
use embassy_executor::SendSpawner;
use hal::get_spawners;

mod ahrs_wrapper;
mod arming;
mod battery_monitor;
mod channel_mapping;
mod crsf;
mod dshot;
mod expo_rates;
mod flight_control;

mod configurator;
mod esc;
mod four_way;
mod gps;
mod hal;
mod icm42688;
mod logging;
mod math_stuff;
mod mixer;
mod model;
mod motors;
mod msp;
mod navigation_utils;
mod osd;
mod pid;
mod shared_state;
mod static_buffer;
mod stored_config;

#[entry]
fn main() -> ! {
    let spawners = get_spawners();

    spawners
        .spawner_high
        .must_spawn(async_main(spawners.spawner_low));

    loop {
        cortex_m::asm::wfi()
    }
}

#[embassy_executor::task]
async fn async_main(spawner_low: SendSpawner) {
    let hardware = make_hardware();
    let flight_exit = flight_control::flight_main(spawner_low, hardware).await;

    match flight_exit {
        flight_control::FlightModeExit::Configurator(flight_mode_exit_data) => {
            configurator_loop(
                flight_mode_exit_data.motor_set,
                flight_mode_exit_data.shared_state,
                flight_mode_exit_data.leds,
                flight_mode_exit_data.phase,
            )
            .await;
        }
        flight_control::FlightModeExit::FourWayEsc(flight_mode_exit_data) => {
            flight_mode_exit_data
                .shared_state
                .push_four_way_mode_parameters(FourWayParameters {
                    leds: flight_mode_exit_data.leds,
                    motors: flight_mode_exit_data.motor_set.into_four_way(),
                });
        }
    }
}
