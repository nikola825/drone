use embassy_executor::SendSpawner;
use embassy_time::{Duration, Ticker};

use crate::logging::info;
use crate::{hal::BatteryMeter, shared_state::SharedState};

#[embassy_executor::task]
async fn battery_monitor_task(mut battery_meter: BatteryMeter, shared_state: &'static SharedState) {
    info!("Battery monitor start");
    let mut ticker = Ticker::every(Duration::from_millis(200));

    loop {
        let measured_battery_voltage = battery_meter.get_voltage();

        shared_state.update_voltage(measured_battery_voltage).await;

        ticker.next().await;
    }
}

pub fn init_battery_monitor(
    battery_meter: BatteryMeter,
    state: &'static SharedState,
    spawner: &SendSpawner,
) {
    spawner.must_spawn(battery_monitor_task(battery_meter, state));
}
