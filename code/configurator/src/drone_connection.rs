use std::fmt::Debug;

use common::{
    configurator_protocol::messages::{BatteryResponse, FcStatus},
    shared_objects::{MotorDirection, StoredConfig},
};

mod mock;
mod serial_msp;

pub mod serial_connection;

pub use mock::MockDroneConnection;
pub use serial_msp::SerialMspConnection;

use crate::drone_connection::serial_connection::SerialError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DroneCommunicationError {
    SerialCommunicationError(SerialError),
    DeserializationFailed,
    WrongMessage(String),
}

pub type DroneCommunicationResult<ResultType> = Result<ResultType, DroneCommunicationError>;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait DroneConnection: Send + Debug {
    async fn test_connection(&mut self) -> bool;

    async fn read_config(&mut self) -> DroneCommunicationResult<StoredConfig>;

    async fn save_config(&mut self, config: StoredConfig) -> DroneCommunicationResult<()>;

    async fn query_motor_count(&mut self) -> DroneCommunicationResult<u8>;

    async fn query_status(&mut self) -> DroneCommunicationResult<FcStatus>;

    async fn query_battery(&mut self) -> DroneCommunicationResult<BatteryResponse>;

    async fn enter_configurator(&mut self) -> DroneCommunicationResult<()>;

    async fn reset_fc(&mut self) -> DroneCommunicationResult<()>;

    async fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: MotorDirection,
    ) -> DroneCommunicationResult<()>;
}
