use common::{
    configurator_protocol::messages::{BatteryResponse, FcStatus},
    shared_objects::{MotorDirection, StoredConfig},
};

mod mock;
mod usb_serial;

pub use mock::MockDroneConnection;
pub use usb_serial::{UsbDroneConnection, UsbPort};

#[allow(dead_code)]
#[derive(Debug)]
pub enum DroneCommunicationError {
    CannotOpenPort(serialport::Error),
    UsbSerialCommunicationError(std::io::Error),
    DeserializationFailed,
    WrongMessage(String),
}

pub trait DroneConnection {
    fn test_connection(&mut self) -> bool;

    fn read_config(&mut self) -> Result<StoredConfig, DroneCommunicationError>;

    fn save_config(&mut self, config: StoredConfig) -> Result<(), DroneCommunicationError>;

    fn query_motor_count(&mut self) -> Result<u8, DroneCommunicationError>;

    fn query_status(&mut self) -> Result<FcStatus, DroneCommunicationError>;

    fn query_battery(&mut self) -> Result<BatteryResponse, DroneCommunicationError>;

    fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError>;

    fn reset_fc(&mut self) -> Result<(), DroneCommunicationError>;

    fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: MotorDirection,
    ) -> Result<(), DroneCommunicationError>;
}
