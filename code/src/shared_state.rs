use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

use crate::{
    arming::ArmingTracker,
    crsf::{CRSFChannels, CRSFFrameLinkStatistics},
};

#[derive(Clone, Default)]
pub struct CommandState {
    pub arming_tracker: ArmingTracker,
    pub commands: CRSFChannels,
}

pub struct SharedState {
    channel_state: Mutex<ThreadModeRawMutex, CommandState>,
    battery_voltage: Mutex<ThreadModeRawMutex, f32>,
    link_state: Mutex<ThreadModeRawMutex, CRSFFrameLinkStatistics>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            channel_state: Mutex::new(CommandState::default()),
            battery_voltage: Mutex::new(0f32),
            link_state: Mutex::new(CRSFFrameLinkStatistics::default()),
        }
    }

    pub async fn command_snapshot(&self) -> CommandState {
        let guard = self.channel_state.lock().await;
        guard.clone()
    }

    pub async fn update_channels(&self, channels: CRSFChannels) {
        let mut guard = self.channel_state.lock().await;
        guard.arming_tracker.update(&channels);
        guard.commands = channels;
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
