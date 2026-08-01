use core::ops::DerefMut;

use common::{
    configurator_protocol::messages::{FcPhase, FcStatus, MotorDirectionSetting},
    shared_objects::StoredConfig,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, signal::Signal, watch::Watch,
};

use crate::{
    arming::ArmingTracker,
    battery_monitor::BatteryInformation,
    crsf::{CRSFChannels, CRSFFrameLinkStatistics},
    four_way::four_way_esc::FourWayParameters,
    gps::GPSState,
    hal::{ConfigStoreType, Leds, ESC_COUNT},
    motor::Motor,
    stored_config::read_stored_config,
};

#[derive(Clone, Default)]
pub struct CommandState {
    pub arming_tracker: ArmingTracker,
    pub commands: CRSFChannels,
}

pub struct SharedState {
    channel_state: Watch<CriticalSectionRawMutex, CommandState, 0>,
    battery_state: Watch<CriticalSectionRawMutex, BatteryInformation, 0>,
    link_state: Watch<CriticalSectionRawMutex, CRSFFrameLinkStatistics, 0>,
    gps_state: Watch<CriticalSectionRawMutex, GPSState, 0>,
    enter_four_way_mode_signal: Watch<CriticalSectionRawMutex, bool, 0>,
    enter_four_way_mode_response: Signal<CriticalSectionRawMutex, FourWayParameters>,
    fc_status: Signal<CriticalSectionRawMutex, FcStatus>,
    config_store: Mutex<CriticalSectionRawMutex, ConfigStoreType>,
    cached_config: Mutex<CriticalSectionRawMutex, Option<StoredConfig>>,
    enter_configurator_mode_signal: Signal<CriticalSectionRawMutex, ()>,
    motor_direction_signal: Signal<CriticalSectionRawMutex, MotorDirectionSetting>,
}

impl SharedState {
    pub fn new(config_store: ConfigStoreType) -> Self {
        SharedState {
            channel_state: Watch::new_with(CommandState::default()),
            battery_state: Watch::new_with(BatteryInformation::default()),
            link_state: Watch::new_with(CRSFFrameLinkStatistics::default()),
            gps_state: Watch::new_with(GPSState::default()),
            enter_four_way_mode_signal: Watch::new_with(false),
            enter_four_way_mode_response: Signal::new(),
            fc_status: Signal::new(),
            config_store: Mutex::new(config_store),
            cached_config: Mutex::new(None),
            enter_configurator_mode_signal: Signal::new(),
            motor_direction_signal: Signal::new(),
        }
    }

    pub fn command_snapshot(&self) -> CommandState {
        self.channel_state.try_get().unwrap()
    }

    pub fn update_channels(&self, channels: CRSFChannels) {
        self.channel_state.sender().send_modify(|state| {
            if let Some(state) = state.as_mut() {
                state.arming_tracker.update(&channels);
                state.commands = channels.clone();
            }
        });
    }

    pub fn update_battery_voltage(&self, voltage: f32) {
        self.battery_state.sender().send_modify(|battery_state| {
            if let Some(state) = battery_state.as_mut() {
                state.update_voltage(voltage);
            }
        });
    }

    pub fn get_battery_voltage(&self) -> f32 {
        self.battery_state.try_get().unwrap().get_total_voltage()
    }

    pub fn get_battery_information(&self) -> BatteryInformation {
        self.battery_state.try_get().unwrap()
    }

    pub fn get_link_state(&self) -> CRSFFrameLinkStatistics {
        self.link_state.try_get().unwrap()
    }

    pub fn update_link_state(&self, link_state: CRSFFrameLinkStatistics) {
        self.link_state.sender().send(link_state);
    }

    pub fn update_gps_state(&self, gps_state: GPSState) {
        self.gps_state.sender().send(gps_state);
    }

    pub fn get_gps_state(&self) -> GPSState {
        self.gps_state.try_get().unwrap()
    }

    pub fn is_four_way_mode_requested(&self) -> bool {
        self.enter_four_way_mode_signal.try_get().unwrap()
    }

    pub fn init_fail(&self, motors: [Motor; ESC_COUNT], leds: Leds, fc_phase: FcPhase) {
        self.push_four_way_mode_parameters(FourWayParameters { motors, leds });
        self.publish_status(FcStatus {
            valid: true,
            fc_phase,
            ..Default::default()
        });
    }

    pub fn push_four_way_mode_parameters(&self, parameters: FourWayParameters) {
        self.enter_four_way_mode_response.signal(parameters);
    }

    pub async fn request_four_way_mode(&self) -> FourWayParameters {
        self.enter_four_way_mode_signal.sender().send(true);

        if let Some(parameters) = self.enter_four_way_mode_response.try_take() {
            return parameters;
        }

        self.enter_four_way_mode_response.wait().await
    }

    pub fn publish_status(&self, mut status: FcStatus) {
        status.valid = true;
        self.fc_status.signal(status);
    }

    pub fn query_status(&self) -> FcStatus {
        self.fc_status.try_take().unwrap_or_default()
    }

    pub async fn read_config(&self) -> StoredConfig {
        let mut cached_config = self.cached_config.lock().await;

        if let Some(cached_config) = cached_config.as_ref() {
            cached_config.clone()
        } else {
            let mut config_store = self.config_store.lock().await;

            let read_config = read_stored_config(config_store.deref_mut()).await;

            if let Ok(config) = read_config {
                *cached_config = Some(config.clone());

                config
            } else {
                StoredConfig::default()
            }
        }
    }

    pub async fn store_config(&self, config: StoredConfig) {
        let mut cached_config = self.cached_config.lock().await;

        let mut config_store = self.config_store.lock().await;
        crate::stored_config::store_config(config_store.deref_mut(), config.clone()).await;

        *cached_config = None;
    }

    pub fn request_configurator_mode(&self) {
        self.enter_configurator_mode_signal.signal(());
    }

    pub fn is_configurator_mode_requested(&self) -> bool {
        self.enter_configurator_mode_signal.signaled()
    }

    pub fn push_motor_direction_setting(&self, motor_override: MotorDirectionSetting) {
        self.motor_direction_signal.signal(motor_override);
    }

    pub fn query_motor_direction_setting(&self) -> Option<MotorDirectionSetting> {
        self.motor_direction_signal.try_take()
    }
}
