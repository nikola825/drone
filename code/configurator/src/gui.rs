use std::{
    fmt::Debug,
    mem,
    sync::{Arc, Mutex},
};

use web_time::Instant;

use common::{
    configurator_protocol::messages::{BatteryResponse, FcStatus},
    shared_objects::{MotorDirection, MotorPosition, StoredConfig},
};

use crate::drone_connection::{DroneCommunicationResult, serial_connection::SerialPort};
use crate::{
    drone_connection::{DroneCommunicationError, DroneConnection},
    time_wrapper::now,
};

pub mod logic;
pub mod visual;

type ExecuteOnGuiCallback = Box<dyn FnOnce(&mut Gui) -> DroneCommunicationResult<()> + Send>;

struct ExecuteOnGuiCallbackWrapper {
    callback: ExecuteOnGuiCallback,
}

impl std::fmt::Debug for ExecuteOnGuiCallbackWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExecuteOnGui")
    }
}

pub enum ConnectionState {
    Disconnected,
    Available(Box<dyn DroneConnection>),
    Taken(web_time::Instant),
}

impl ConnectionState {
    pub fn take(&mut self) -> Option<Box<dyn DroneConnection>> {
        if matches!(self, Self::Available(_)) {
            let old_state = mem::replace(self, Self::Taken(web_time::Instant::now()));
            match old_state {
                ConnectionState::Disconnected => None,
                ConnectionState::Available(drone_connection) => Some(drone_connection),
                ConnectionState::Taken(_) => None,
            }
        } else {
            None
        }
    }

    pub fn release(&mut self, connection: Box<dyn DroneConnection>) {
        *self = ConnectionState::Available(connection)
    }

    pub fn taken_time(&self) -> Option<Instant> {
        if let ConnectionState::Taken(time) = self {
            Some(*time)
        } else {
            None
        }
    }
}

pub struct ConnectionOwnership {
    state: Arc<Mutex<ConnectionState>>,
    connection: Option<Box<dyn DroneConnection>>,
    disconnected: bool,
}

impl ConnectionOwnership {
    fn new(state: Arc<Mutex<ConnectionState>>, connection: Box<dyn DroneConnection>) -> Self {
        Self {
            state,
            connection: Some(connection),
            disconnected: false,
        }
    }

    fn disconnect(&mut self) {
        self.disconnected = true;
    }
}

impl Debug for ConnectionOwnership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionOwnership")
    }
}

impl Drop for ConnectionOwnership {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            if self.disconnected {
                *self.state.lock().unwrap() = ConnectionState::Disconnected;
            } else {
                self.state.lock().unwrap().release(connection);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl DroneConnection for ConnectionOwnership {
    async fn test_connection(&mut self) -> bool {
        self.connection.as_mut().unwrap().test_connection().await
    }

    async fn read_config(&mut self) -> Result<StoredConfig, DroneCommunicationError> {
        self.connection.as_mut().unwrap().read_config().await
    }

    async fn save_config(&mut self, config: StoredConfig) -> Result<(), DroneCommunicationError> {
        self.connection.as_mut().unwrap().save_config(config).await
    }

    async fn query_motor_count(&mut self) -> Result<u8, DroneCommunicationError> {
        self.connection.as_mut().unwrap().query_motor_count().await
    }

    async fn query_status(&mut self) -> Result<FcStatus, DroneCommunicationError> {
        self.connection.as_mut().unwrap().query_status().await
    }

    async fn query_battery(&mut self) -> Result<BatteryResponse, DroneCommunicationError> {
        self.connection.as_mut().unwrap().query_battery().await
    }

    async fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        self.connection.as_mut().unwrap().enter_configurator().await
    }

    async fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        self.connection.as_mut().unwrap().reset_fc().await
    }

    async fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: MotorDirection,
    ) -> Result<(), DroneCommunicationError> {
        self.connection
            .as_mut()
            .unwrap()
            .drive_motor(motor_index, direction)
            .await
    }
}

#[derive(Debug)]
enum Message {
    RefreshPorts,
    ConnectToPort,
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
    ExecuteOnGui(ExecuteOnGuiCallbackWrapper),
}

pub struct Gui {
    motor_settings: MotorSettings,
    port_picker: SerialPortPicker,
    message_box_message: Option<String>,
    connection: Arc<Mutex<ConnectionState>>,
    state: SharedState,
    sender: std::sync::mpsc::Sender<Message>,
    receiver: std::sync::mpsc::Receiver<Message>,
    last_tick: Instant,
    loading_screen: bool,
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
            last_fc_status_update: now(),
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
    Serial(SerialPort),
    Mock,
}

impl From<SerialPort> for AvailablePort {
    fn from(value: SerialPort) -> Self {
        Self::Serial(value)
    }
}

impl std::fmt::Display for AvailablePort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailablePort::Serial(usb_port) => std::fmt::Display::fmt(usb_port, f),
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
