use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

use crate::crsf::{CRSFChannels, CRSFFrameLinkStatistics};
pub struct Store {
    pub channel_state: Mutex<ThreadModeRawMutex, CRSFChannels>,
    pub battery_voltage: Mutex<ThreadModeRawMutex, f32>,
    pub link_state: Mutex<ThreadModeRawMutex, CRSFFrameLinkStatistics>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            channel_state: Mutex::new(CRSFChannels::default()),
            battery_voltage: Mutex::new(0f32),
            link_state: Mutex::new(CRSFFrameLinkStatistics::default()),
        }
    }

    pub async fn channel_snapshot(&self) -> CRSFChannels {
        let guard = self.channel_state.lock().await;
        guard.clone()
    }

    pub async fn update_channels(&self, channels: CRSFChannels) {
        let mut guard = self.channel_state.lock().await;
        *guard = channels;
    }

    pub async fn update_voltage(&self, voltage: f32) {
        let mut guard = self.battery_voltage.lock().await;
        *guard = voltage;
    }

    pub async fn get_voltage(&self) -> f32 {
        let guard = self.battery_voltage.lock().await;
        *guard
    }

    pub async fn get_link_state(&self) -> CRSFFrameLinkStatistics {
        let guard = self.link_state.lock().await;
        guard.clone()
    }

    pub async fn update_link_state(&self, link_state: CRSFFrameLinkStatistics) {
        let mut guard = self.link_state.lock().await;
        *guard = link_state;
    }
}
