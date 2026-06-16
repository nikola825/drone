#![no_std]
#![no_main]

use crate::hal::make_hardware;
use cortex_m_rt::entry;
use embassy_executor::SendSpawner;
use hal::get_spawners;

mod ahrs_wrapper;
mod arming;
mod battery_monitor;
mod channel_mapping;
mod crc8;
mod crsf;
mod dshot;
mod expo_rates;
mod flight_control;
#[cfg(feature = "serial-passthrough")]
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
mod nopdelays;
mod osd;
mod pid;
mod shared_state;
mod static_buffer;
mod stored_config;

#[entry]
fn main() -> ! {
    let spawners = get_spawners();

    spawners
        .spawner_low
        .must_spawn(async_main(spawners.spawner_low, spawners.spawner_high));

    loop {
        cortex_m::asm::wfi()
    }
}

#[embassy_executor::task]
async fn async_main(spawner_low: SendSpawner, spawner_high: SendSpawner) {
    let hardware = make_hardware();

    #[cfg(feature = "serial-passthrough")]
    {
        use crate::msp::usb_passthrough::serial_passthrough_main;
        serial_passthrough_main(&spawner_high, hardware).await;
    }

    flight_control::flight_main(spawner_low, spawner_high, hardware).await;
}
