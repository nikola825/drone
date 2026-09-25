#[cfg(feature = "debug-logging")]
use common::shared_objects::DebugLog;
use common::{
    configurator_protocol::messages::{FcStatus, MotorDirectionSetting},
    shared_objects::StoredConfig,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, signal::Signal, watch::Watch,
};

#[cfg(feature = "debug-logging")]
use crate::debug_logging::LoggingState;
use crate::{
    arming::ArmingTracker,
    battery_monitor::BatteryInformation,
    crsf::{CRSFChannels, CRSFFrameLinkStatistics},
    four_way::four_way_esc::FourWayParameters,
    gps::GPSState,
    hal::ConfigStoreType,
    stored_config::read_stored_config,
};

#[derive(Clone)]
pub struct CommandState {
    pub arming_tracker: ArmingTracker,
    pub commands: CRSFChannels,
}

impl CommandState {
    pub const fn new() -> Self {
        Self {
            arming_tracker: ArmingTracker::new(),
            commands: CRSFChannels::new(),
        }
    }
}

pub static SHARED_STATE: SharedStateImpl = SharedStateImpl::new();

pub struct SharedStateImpl {
    channel_state: Watch<CriticalSectionRawMutex, CommandState, 0>,
    battery_state: Watch<CriticalSectionRawMutex, BatteryInformation, 0>,
    link_state: Watch<CriticalSectionRawMutex, CRSFFrameLinkStatistics, 0>,
    gps_state: Watch<CriticalSectionRawMutex, GPSState, 0>,
    enter_four_way_mode_signal: Watch<CriticalSectionRawMutex, bool, 0>,
    enter_four_way_mode_response: Signal<CriticalSectionRawMutex, FourWayParameters>,
    fc_status: Signal<CriticalSectionRawMutex, FcStatus>,
    config_store: Mutex<CriticalSectionRawMutex, Option<ConfigStoreType>>,
    cached_config: Mutex<CriticalSectionRawMutex, Option<StoredConfig>>,
    enter_configurator_mode_signal: Signal<CriticalSectionRawMutex, ()>,
    motor_direction_signal: Signal<CriticalSectionRawMutex, MotorDirectionSetting>,

    #[cfg(feature = "debug-logging")]
    debug_logs: Mutex<CriticalSectionRawMutex, LoggingState>,
}

impl SharedStateImpl {
    pub const fn new() -> Self {
        SharedStateImpl {
            channel_state: Watch::new_with(CommandState::new()),
            battery_state: Watch::new_with(BatteryInformation::new()),
            link_state: Watch::new_with(CRSFFrameLinkStatistics::new()),
            gps_state: Watch::new_with(GPSState::new()),
            enter_four_way_mode_signal: Watch::new_with(false),
            enter_four_way_mode_response: Signal::new(),
            fc_status: Signal::new(),
            config_store: Mutex::new(None),
            cached_config: Mutex::new(None),
            enter_configurator_mode_signal: Signal::new(),
            motor_direction_signal: Signal::new(),
            #[cfg(feature = "debug-logging")]
            debug_logs: Mutex::new(LoggingState::new()),
        }
    }

    pub async fn init(&self, config_store: ConfigStoreType) {
        let mut store = self.config_store.lock().await;
        *store = Some(config_store);
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

            let read_config = read_stored_config(config_store.as_mut().unwrap()).await;

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
        crate::stored_config::store_config(config_store.as_mut().unwrap(), config.clone()).await;

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

    #[cfg(feature = "debug-logging")]
    #[allow(dead_code)]
    pub async fn write_log_fmt(&self, args: core::fmt::Arguments<'_>) {
        use core::fmt::Write;
        let mut logging_state = self.debug_logs.lock().await;

        let _ = logging_state.write_fmt(args);
    }

    #[cfg(feature = "debug-logging")]
    pub async fn get_log(&self) -> Option<DebugLog> {
        let mut logging_state = self.debug_logs.lock().await;

        logging_state.take_log()
    }
}
