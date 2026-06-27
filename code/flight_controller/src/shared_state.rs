use core::ops::DerefMut;

use common::{
    configurator_protocol::messages::{FcPhase, FcStatus, MotorOverride},
    shared_objects::StoredConfig,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, signal::Signal};

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
    channel_state: Mutex<CriticalSectionRawMutex, CommandState>,
    battery_state: Mutex<CriticalSectionRawMutex, BatteryInformation>,
    link_state: Mutex<CriticalSectionRawMutex, CRSFFrameLinkStatistics>,
    gps_state: Mutex<CriticalSectionRawMutex, GPSState>,
    enter_four_way_mode_signal: Signal<CriticalSectionRawMutex, ()>,
    enter_four_way_mode_response: Signal<CriticalSectionRawMutex, FourWayParameters>,
    fc_status: Signal<CriticalSectionRawMutex, FcStatus>,
    config_store: Mutex<CriticalSectionRawMutex, ConfigStoreType>,
    cached_config: Mutex<CriticalSectionRawMutex, Option<StoredConfig>>,
    enter_configurator_mode_signal: Signal<CriticalSectionRawMutex, ()>,
    motor_override_signal: Signal<CriticalSectionRawMutex, MotorOverride>,
}

impl SharedState {
    pub fn new(config_store: ConfigStoreType) -> Self {
        SharedState {
            channel_state: Mutex::new(CommandState::default()),
            battery_state: Mutex::new(BatteryInformation::default()),
            link_state: Mutex::new(CRSFFrameLinkStatistics::default()),
            gps_state: Mutex::new(GPSState::default()),
            enter_four_way_mode_signal: Signal::new(),
            enter_four_way_mode_response: Signal::new(),
            fc_status: Signal::new(),
            config_store: Mutex::new(config_store),
            cached_config: Mutex::new(None),
            enter_configurator_mode_signal: Signal::new(),
            motor_override_signal: Signal::new(),
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

    pub async fn update_gps_state(&self, gps_state: GPSState) {
        let mut guard = self.gps_state.lock().await;
        *guard = gps_state;
    }

    pub async fn get_gps_state(&self) -> GPSState {
        let guard = self.gps_state.lock().await;
        guard.clone()
    }

    pub fn is_four_way_mode_requested(&self) -> bool {
        self.enter_four_way_mode_signal.signaled()
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
        self.enter_four_way_mode_signal.signal(());

        if let Some(parameters) = self.enter_four_way_mode_response.try_take() {
            return parameters;
        }

        self.enter_four_way_mode_response.wait().await
    }

    pub fn publish_status(&self, mut status: FcStatus) {
        status.valid = true;
        self.fc_status.signal(status);
    }

    pub async fn query_status(&self) -> FcStatus {
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

        *cached_config = Some(config);
    }

    pub fn request_configurator_mode(&self) {
        self.enter_configurator_mode_signal.signal(());
    }

    pub fn is_configurator_mode_requested(&self) -> bool {
        self.enter_configurator_mode_signal.signaled()
    }

    pub fn push_motor_override(&self, motor_override: MotorOverride) {
        self.motor_override_signal.signal(motor_override);
    }

    pub fn query_motor_override(&self) -> Option<MotorOverride> {
        self.motor_override_signal.try_take()
    }
}
