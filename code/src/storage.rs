use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

use crate::crsf::CRSFChannels;
pub struct Store {
    pub channel_state: Mutex<ThreadModeRawMutex, ChannelState>,
    pub battery_state: Mutex<ThreadModeRawMutex, BatteryState>,
}

#[derive(Clone, Default)]
pub struct ChannelState {
    pub channels: CRSFChannels,
}

#[derive(Clone, Default)]
pub struct BatteryState {
    pub voltage: f32,
}

impl Store {
    pub fn new() -> Self {
        Store {
            channel_state: Mutex::new(ChannelState::default()),
            battery_state: Mutex::new(BatteryState::default()),
        }
    }

    pub async fn channel_snapshot(&self) -> ChannelState {
        let guard = self.channel_state.lock().await;
        guard.clone()
    }

    pub async fn update_channels(&self, channels: CRSFChannels) {
        let mut guard = self.channel_state.lock().await;
        guard.channels = channels;
    }

    pub async fn update_voltage(&self, voltage: f32) {
        let mut guard = self.battery_state.lock().await;
        guard.voltage = voltage;
    }

    pub async fn get_voltage(&self) -> f32 {
        let guard = self.battery_state.lock().await;
        guard.voltage
    }
}
