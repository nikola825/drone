use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};

use crate::logging::info;
use crate::{hw_select::AdcReader, shared_state::SharedState};

#[embassy_executor::task]
async fn battery_monitor_task(mut adc_reader: AdcReader, shared_state: &'static SharedState) {
    info!("Battery monitor start");
    let mut ticker = Ticker::every(Duration::from_millis(200));

    loop {
        let measured_battery_voltage = adc_reader.get_bat();

        shared_state.update_voltage(measured_battery_voltage).await;

        ticker.next().await;
    }
}

pub fn init_battery_monitor(adc_reader: AdcReader, state: &'static SharedState, spawner: &Spawner) {
    spawner
        .spawn(battery_monitor_task(adc_reader, state))
        .unwrap();
}
