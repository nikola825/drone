use embassy_time::{Duration, Ticker};

use crate::logging::info;
use crate::{hw_select::AdcReader, storage::Store};

#[embassy_executor::task]
pub async fn battery_monitor_task(mut adc_reader: AdcReader, storage: &'static Store) {
    info!("Battery monitor start");
    let mut ticker = Ticker::every(Duration::from_millis(200));

    loop {
        let measured_battery_voltage = adc_reader.get_bat();

        storage.update_voltage(measured_battery_voltage).await;

        info!("BAT {}", measured_battery_voltage);

        ticker.next().await;
    }
}
