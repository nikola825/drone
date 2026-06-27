use std::time::Duration;

use common::{
    configurator_protocol::{
        messages::{BatteryResponse, FcStatus, MotorOverride},
        protocol::{ConfiguratorMessage, Dummy},
    },
    msp_protocol::{
        messages::{ApiVersionMessage, MotorConfigMessage},
        protocol::{MSPMessage, MSPMessagePayload, MSPReceivedPayload},
    },
    shared_objects::StoredConfig,
};
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use zerocopy::{IntoBytes, TryFromBytes, TryReadError};

const ALLOWED_VIDS: [u16; 2] = [0x0483, 0xdead];
const ALLOWED_PIDS: [u16; 1] = [0xbeef];

#[derive(Debug)]
pub enum DroneCommunicationError {
    PortOpenError(serialport::Error),
    UsbSerialCommunicationError(std::io::Error),
    DeserializationError,
}

impl From<std::io::Error> for DroneCommunicationError {
    fn from(error: std::io::Error) -> Self {
        Self::UsbSerialCommunicationError(error)
    }
}

impl From<serialport::Error> for DroneCommunicationError {
    fn from(error: serialport::Error) -> Self {
        Self::PortOpenError(error)
    }
}

impl<Payload: MSPReceivedPayload> From<TryReadError<&[u8], Payload>> for DroneCommunicationError {
    fn from(_: TryReadError<&[u8], Payload>) -> Self {
        Self::DeserializationError
    }
}

pub struct UsbPort {
    port_info: SerialPortInfo,
}

impl UsbPort {
    pub fn enumerate() -> Result<Vec<UsbPort>, DroneCommunicationError> {
        let available_ports = serialport::available_ports()?;

        let filtered = available_ports.into_iter().filter_map(|port| {
            if let SerialPortType::UsbPort(usb_info) = &port.port_type {
                if ALLOWED_VIDS.iter().any(|x| *x == usb_info.vid)
                    && ALLOWED_PIDS.iter().any(|x| *x == usb_info.pid)
                {
                    return Some(UsbPort { port_info: port });
                }
            }

            return None;
        });

        Ok(filtered.collect())
    }

    pub fn description(&self) -> String {
        format!("{}", self.port_info.port_name)
    }
}

pub struct DroneConnection {
    port: Box<dyn SerialPort>,
}

impl DroneConnection {
    pub fn new(port: &UsbPort) -> Result<Self, DroneCommunicationError> {
        println!("Connecting to {:?}", port.description());

        let connection = serialport::new(port.port_info.port_name.clone(), 115200)
            .timeout(Duration::from_secs(1))
            .open()?;

        Ok(Self { port: connection })
    }

    pub fn reset_connection(&mut self) -> Result<(), DroneCommunicationError> {
        // flush anything in the buffers
        let mut dummy: Vec<u8> = Vec::new();
        let _ = self.port.read_to_end(&mut dummy);

        Ok(())
    }

    fn send_msp_request_no_response<RequestPayload: MSPMessagePayload>(
        &mut self,
        request: MSPMessage<RequestPayload>,
    ) -> Result<(), DroneCommunicationError> {
        self.port.write_all(request.as_bytes())?;

        Ok(())
    }

    fn send_msp_request<RequestPayload: MSPMessagePayload, ResponsePayload: MSPReceivedPayload>(
        &mut self,
        request: MSPMessage<RequestPayload>,
    ) -> Result<MSPMessage<ResponsePayload>, DroneCommunicationError> {
        self.port.write_all(request.as_bytes())?;

        let mut response_buffer = [0u8; 280];

        self.port
            .read_exact(&mut response_buffer[..size_of::<MSPMessage<ResponsePayload>>()])?;

        let parse_result = MSPMessage::try_read_from_prefix(&response_buffer);

        if let Ok(response) = parse_result {
            Ok(response.0)
        } else {
            Err(DroneCommunicationError::DeserializationError)
        }
    }

    fn send_configurator_request(
        &mut self,
        request: ConfiguratorMessage,
    ) -> Result<ConfiguratorMessage, DroneCommunicationError> {
        let response: MSPMessage<ConfiguratorMessage> = self.send_msp_request(request.into())?;

        Ok(response.payload)
    }

    pub fn test_connection(&mut self) -> bool {
        if let Err(err) = self.reset_connection() {
            println!("USB reset failed {:?}", err);
            return false;
        }

        let api_version_response = self.send_msp_request(ApiVersionMessage::default().into());

        match api_version_response {
            Ok(api_version) => {
                let api_version: ApiVersionMessage = api_version.payload;
                println!(
                    "Connection success {} {} {}",
                    api_version.api_version_major,
                    api_version.api_version_minor,
                    api_version.protocol_version,
                );

                true
            }
            Err(err) => {
                println!("API version query failed {:?}", err);
                false
            }
        }
    }

    pub fn read_config(&mut self) -> Result<StoredConfig, DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::QueryConfig(Dummy::default()))
            .unwrap();

        if let ConfiguratorMessage::QueryConfigResponse(cfg) = response {
            return Ok(cfg);
        } else {
            return Err(DroneCommunicationError::DeserializationError);
        }
    }

    pub fn query_motor_count(&mut self) -> Result<u8, DroneCommunicationError> {
        let response: MSPMessage<MotorConfigMessage> =
            self.send_msp_request(MotorConfigMessage::new(0).into())?;

        Ok(response.payload.motor_count)
    }

    pub fn query_status(&mut self) -> Result<FcStatus, DroneCommunicationError> {
        let response =
            self.send_configurator_request(ConfiguratorMessage::QueryFcStatus(Dummy::default()))?;

        if let ConfiguratorMessage::QueryFcStatusResponse(status) = response {
            return Ok(status.inner);
        } else {
            panic!("FAIL");
        }
    }

    pub fn query_battery(&mut self) -> Result<BatteryResponse, DroneCommunicationError> {
        let response =
            self.send_configurator_request(ConfiguratorMessage::QueryBattery(Dummy::default()))?;

        if let ConfiguratorMessage::QueryBatteryResponse(battery) = response {
            return Ok(battery.inner);
        } else {
            panic!("FAIL");
        }
    }

    pub fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::StartConfigurator(Dummy::default()))?;

        if let ConfiguratorMessage::EmptyOkResponse(_) = response {
            return Ok(());
        } else {
            panic!("FAIL");
        }
    }

    pub fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        self.send_msp_request_no_response(ConfiguratorMessage::ResetFc(Dummy::default()).into())?;

        Ok(())
    }

    pub fn drive_forward(&mut self, motor_index: u8) -> Result<(), DroneCommunicationError> {
        self.send_configurator_request(ConfiguratorMessage::OverrideMotor(
            MotorOverride {
                direction: common::configurator_protocol::messages::MotorOverrideDirection::Forward,
                motor_index,
            }
            .into(),
        ))?;

        Ok(())
    }

    pub fn drive_backward(&mut self, motor_index: u8) -> Result<(), DroneCommunicationError> {
        self.send_configurator_request(ConfiguratorMessage::OverrideMotor(
            MotorOverride {
                direction:
                    common::configurator_protocol::messages::MotorOverrideDirection::Backward,
                motor_index: motor_index,
            }
            .into(),
        ))?;

        Ok(())
    }
}
