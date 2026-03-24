use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

use crate::{
    arming::ArmingTracker,
    battery_monitor::BatteryInformation,
    crsf::{CRSFChannels, CRSFFrameLinkStatistics}, navigation::NavigationState,
};

#[derive(Clone, Default)]
pub struct CommandState {
    pub arming_tracker: ArmingTracker,
    pub commands: CRSFChannels,
}

pub struct SharedState {
    channel_state: Mutex<CriticalSectionRawMutex, CommandState>,
    battery_state: Mutex<CriticalSectionRawMutex, BatteryInformation>,
    link_state: Mutex<CriticalSectionRawMutex, CRSFFrameLinkStatistics>,
    navigation_state: Mutex<CriticalSectionRawMutex, NavigationState>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            channel_state: Mutex::new(CommandState::default()),
            battery_state: Mutex::new(BatteryInformation::default()),
            link_state: Mutex::new(CRSFFrameLinkStatistics::default()),
            navigation_state: Mutex::new(NavigationState::default()),
        }
    }

    pub async fn command_snapshot(&self) -> CommandState {
        let guard = self.channel_state.lock().await;
        guard.clone()
    }

    pub async fn is_armed(&self) -> bool {
        self.channel_state.lock().await.arming_tracker.is_armed()
    }

    pub async fn update_channels(&self, channels: CRSFChannels) {
        let mut guard = self.channel_state.lock().await;
        guard.arming_tracker.update(&channels);
        guard.commands = channels;
    }

    pub async fn update_battery_voltage(&self, voltage: f32) {
        let mut guard = self.battery_state.lock().await;
        guard.update_voltage(voltage);
    }

    pub async fn get_battery_voltage(&self) -> f32 {
        let guard = self.battery_state.lock().await;
        guard.get_total_voltage()
    }

    pub async fn get_battery_information(&self) -> BatteryInformation {
        let guard = self.battery_state.lock().await;
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

    pub async fn update_navigation_state(&self, navigation_state: NavigationState) {
        let mut guard = self.navigation_state.lock().await;
        (*guard) = navigation_state;
    }

    pub async fn get_navigation_state(&self) -> NavigationState {
        let guard = self.navigation_state.lock().await;
        guard.clone()
    }
}
