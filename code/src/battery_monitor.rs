use embassy_executor::SendSpawner;
use embassy_time::{Duration, Ticker};

use crate::logging::info;
use crate::{hal::BatteryMeter, shared_state::SharedState};

const DEFAULT_CELL_COUNT: u8 = 4;

fn estimate_cell_count(voltage: f32) -> u8 {
    if voltage >= 16.8 {
        6
    } else if voltage > 13.2 {
        4
    } else {
        3
    }
}

#[derive(Clone, Copy)]
pub struct BatteryInformation {
    voltage: f32,
    cell_count: Option<u8>,
}

impl Default for BatteryInformation {
    fn default() -> Self {
        Self {
            voltage: 0f32,
            cell_count: None,
        }
    }
}

impl BatteryInformation {
    pub fn update_voltage(&mut self, voltage: f32) {
        self.voltage = voltage;

        if self.cell_count.is_none() {
            self.cell_count = Some(estimate_cell_count(voltage))
        }
    }

    pub fn get_cell_count(&self) -> u8 {
        *self.cell_count.as_ref().unwrap_or(&DEFAULT_CELL_COUNT)
    }

    pub fn get_cell_voltage(&self) -> f32 {
        self.voltage / (self.get_cell_count() as f32)
    }

    pub fn get_total_voltage(&self) -> f32 {
        self.voltage
    }
}

#[embassy_executor::task]
async fn battery_monitor_task(mut battery_meter: BatteryMeter, shared_state: &'static SharedState) {
    info!("Battery monitor start");
    let mut ticker = Ticker::every(Duration::from_millis(200));

    loop {
        let measured_battery_voltage = battery_meter.get_voltage();

        shared_state
            .update_battery_voltage(measured_battery_voltage)
            .await;

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
