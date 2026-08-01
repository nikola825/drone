use std::time::Instant;

use common::{
    configurator_protocol::messages::{BatteryResponse, FcStatus},
    shared_objects::{MotorDirection, MotorPosition, StoredConfig},
};

use crate::drone_connection::{DroneConnection, UsbPort};

pub mod logic;
pub mod visual;

#[derive(Clone, Debug)]
enum Message {
    RefreshPorts,
    ConnectToPort,
    ChosePort(AvailablePort),
    Tick,
    EnterConfigurator,
    ResetFc,
    MotorDirectionChosen(u8, MotorDirection),
    MotorPositionChosen(u8, MotorPosition),
    ReadConfig,
    SaveConfig,
    DismissMessageBox,
    YawOffsetchanged(String),
    PitchOffsetChanged(String),
    RollOffsetChanged(String),
}

#[derive(Default)]
pub struct Gui {
    motor_settings: MotorSettings,
    port_picker: SerialPortPicker,
    message_box_message: Option<String>,
    connection: Option<Box<dyn DroneConnection>>,
    state: SharedState,
}

struct SharedState {
    last_fc_status_update: Instant,
    last_fc_status: FcStatus,
    battery_status: BatteryResponse,
    connected: bool,
    config: CachedConfig,
    motor_count: u8,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            last_fc_status_update: Instant::now(),
            battery_status: Default::default(),
            connected: false,
            last_fc_status: Default::default(),
            config: Default::default(),
            motor_count: 0,
        }
    }
}

#[derive(Default)]
struct SerialPortPicker {
    available_ports: Vec<AvailablePort>,
    selected: Option<AvailablePort>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AvailablePort {
    Usb(UsbPort),
    Mock,
}

impl From<UsbPort> for AvailablePort {
    fn from(value: UsbPort) -> Self {
        Self::Usb(value)
    }
}

impl std::fmt::Display for AvailablePort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailablePort::Usb(usb_port) => std::fmt::Display::fmt(usb_port, f),
            AvailablePort::Mock => write!(f, "Mock"),
        }
    }
}

#[derive(Default)]
struct MotorSettings {
    yaw_offset_string: String,
    pitch_offset_string: String,
    roll_offset_string: String,
}

struct InnerCachedConfig {
    read_config: StoredConfig,
    current_config: StoredConfig,
    dirty: bool,
}

#[derive(Default)]
struct CachedConfig {
    inner: Option<InnerCachedConfig>,
}

impl From<StoredConfig> for CachedConfig {
    fn from(config: StoredConfig) -> Self {
        Self {
            inner: Some(InnerCachedConfig {
                read_config: config.clone(),
                current_config: config.clone(),
                dirty: !config.valid_checksum(),
            }),
        }
    }
}
