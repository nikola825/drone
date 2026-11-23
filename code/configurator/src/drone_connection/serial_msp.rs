use std::time::Duration;

use common::{
    configurator_protocol::{
        messages::{BatteryResponse, FcStatus, MotorDirectionSetting},
        protocol::{ConfiguratorMessage, Dummy},
    },
    msp_protocol::{
        messages::{ApiVersionMessage, MotorConfigMessage},
        protocol::{MSPMessage, MSPMessagePayload, MSPReceivedPayload},
    },
    shared_objects::{MotorDirection, StoredConfig},
};
use zerocopy::{IntoBytes, TryFromBytes, TryReadError};

use crate::drone_connection::{
    DroneCommunicationError, DroneConnection,
    serial_connection::{SerialConnection, SerialError},
};

impl From<SerialError> for DroneCommunicationError {
    fn from(error: SerialError) -> Self {
        Self::SerialCommunicationError(error)
    }
}

impl<Payload: MSPReceivedPayload> From<TryReadError<&[u8], Payload>> for DroneCommunicationError {
    fn from(_: TryReadError<&[u8], Payload>) -> Self {
        Self::DeserializationFailed
    }
}

pub struct SerialMspConnection {
    port: SerialConnection,
}

impl std::fmt::Debug for SerialMspConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerialMspConnection")
    }
}

impl SerialMspConnection {
    pub fn new(port: SerialConnection) -> Self {
        Self { port }
    }

    async fn reset_connection(&mut self) -> Result<(), DroneCommunicationError> {
        // flush anything in the buffers
        let _ = self.port.read_to_end().await;
        Ok(())
    }

    async fn send_msp_request_no_response<RequestPayload: MSPMessagePayload>(
        &mut self,
        request: MSPMessage<RequestPayload>,
    ) -> Result<(), DroneCommunicationError> {
        self.port.write_all(request.as_bytes()).await?;

        Ok(())
    }

    async fn send_msp_request<
        RequestPayload: MSPMessagePayload,
        ResponsePayload: MSPReceivedPayload,
    >(
        &mut self,
        request: MSPMessage<RequestPayload>,
    ) -> Result<MSPMessage<ResponsePayload>, DroneCommunicationError> {
        self.port.write_all(request.as_bytes()).await?;

        let mut response_buffer = [0u8; 280];

        self.port
            .read_exact(&mut response_buffer[..size_of::<MSPMessage<ResponsePayload>>()])
            .await?;

        let parse_result = MSPMessage::try_read_from_prefix(&response_buffer);

        if let Ok(response) = parse_result {
            Ok(response.0)
        } else {
            Err(DroneCommunicationError::DeserializationFailed)
        }
    }

    async fn send_configurator_request(
        &mut self,
        request: ConfiguratorMessage,
    ) -> Result<ConfiguratorMessage, DroneCommunicationError> {
        let response: MSPMessage<ConfiguratorMessage> =
            self.send_msp_request(request.into()).await?;

        Ok(response.payload)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl DroneConnection for SerialMspConnection {
    async fn test_connection(&mut self) -> bool {
        if let Err(err) = self.reset_connection().await {
            println!("USB reset failed {:?}", err);
            return false;
        }
        let api_version_response = self
            .send_msp_request(ApiVersionMessage::default().into())
            .await;

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

    async fn read_config(&mut self) -> Result<StoredConfig, DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::QueryConfig(Dummy::default()))
            .await?;

        if let ConfiguratorMessage::QueryConfigResponse(cfg) = response {
            Ok(cfg)
        } else {
            Err(DroneCommunicationError::WrongMessage(
                "Drone sent wrong response type when querying stored config".into(),
            ))
        }
    }

    async fn save_config(&mut self, config: StoredConfig) -> Result<(), DroneCommunicationError> {
        // Flash storage can take some time
        self.port.set_timeout(Duration::from_secs(5)).await?;

        let response = self
            .send_configurator_request(ConfiguratorMessage::SetConfig(config))
            .await?;

        self.port.set_timeout(Duration::from_secs(1)).await?;

        if let ConfiguratorMessage::EmptyOkResponse(_) = response {
            Ok(())
        } else {
            Err(DroneCommunicationError::WrongMessage(
                "Drone sent wrong response type when storing config".into(),
            ))
        }
    }

    async fn query_motor_count(&mut self) -> Result<u8, DroneCommunicationError> {
        let response: MSPMessage<MotorConfigMessage> = self
            .send_msp_request(MotorConfigMessage::new(0).into())
            .await?;

        Ok(response.payload.motor_count)
    }

    async fn query_status(&mut self) -> Result<FcStatus, DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::QueryFcStatus(Dummy::default()))
            .await?;

        if let ConfiguratorMessage::QueryFcStatusResponse(status) = response {
            Ok(status.inner)
        } else {
            Err(DroneCommunicationError::WrongMessage(
                "Drone sent wrong response type when querying FC status".into(),
            ))
        }
    }

    async fn query_battery(&mut self) -> Result<BatteryResponse, DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::QueryBattery(Dummy::default()))
            .await?;

        if let ConfiguratorMessage::QueryBatteryResponse(battery) = response {
            Ok(battery.inner)
        } else {
            Err(DroneCommunicationError::WrongMessage(
                "Drone sent wrong response type when querying battery".into(),
            ))
        }
    }

    async fn enter_configurator(&mut self) -> Result<(), DroneCommunicationError> {
        let response = self
            .send_configurator_request(ConfiguratorMessage::StartConfigurator(Dummy::default()))
            .await?;

        if let ConfiguratorMessage::EmptyOkResponse(_) = response {
            Ok(())
        } else {
            Err(DroneCommunicationError::WrongMessage(
                "Drone sent wrong response type when starting config mode".into(),
            ))
        }
    }

    async fn reset_fc(&mut self) -> Result<(), DroneCommunicationError> {
        self.send_msp_request_no_response(ConfiguratorMessage::ResetFc(Dummy::default()).into())
            .await?;

        Ok(())
    }

    async fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: MotorDirection,
    ) -> Result<(), DroneCommunicationError> {
        self.send_configurator_request(ConfiguratorMessage::SetMotorDirection(
            MotorDirectionSetting {
                direction,
                motor_index,
            }
            .into(),
        ))
        .await?;

        Ok(())
    }
}
