use common::{
    configurator_protocol::messages::{BatteryResponse, FcPhase, FcStatus},
    shared_objects::StoredConfig,
};
use log::info;

use crate::drone_connection::DroneConnection;

pub struct MockDroneConnection {
    config: StoredConfig,
    status: FcStatus,
}

impl std::fmt::Debug for MockDroneConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockDroneConnection")
    }
}

impl MockDroneConnection {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            status: FcStatus {
                valid: true,
                total_duration: 30f32.into(),
                inner_duration: 40f32.into(),
                min_measured_period: 1001.into(),
                max_measured_period: 1002.into(),
                fc_phase: FcPhase::Armed,
            },
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl DroneConnection for MockDroneConnection {
    async fn test_connection(&mut self) -> bool {
        true
    }

    async fn read_config(
        &mut self,
    ) -> Result<common::shared_objects::StoredConfig, super::DroneCommunicationError> {
        info!("Read config {:?}", self.config);
        Ok(self.config.clone())
    }

    async fn save_config(
        &mut self,
        mut config: common::shared_objects::StoredConfig,
    ) -> Result<(), super::DroneCommunicationError> {
        info!("Save config {:?}", config);
        config.update_checksum();
        self.config = config;

        Ok(())
    }

    async fn query_motor_count(&mut self) -> Result<u8, super::DroneCommunicationError> {
        info!("Query motor count");
        Ok(4)
    }

    async fn query_status(
        &mut self,
    ) -> Result<common::configurator_protocol::messages::FcStatus, super::DroneCommunicationError>
    {
        info!("Query status");
        Ok(self.status)
    }

    async fn query_battery(
        &mut self,
    ) -> Result<
        common::configurator_protocol::messages::BatteryResponse,
        super::DroneCommunicationError,
    > {
        info!("Query battery");
        Ok(BatteryResponse {
            voltage: 15f32.into(),
            cell_count: 4,
        })
    }

    async fn enter_configurator(&mut self) -> Result<(), super::DroneCommunicationError> {
        self.status.fc_phase = FcPhase::Config;
        info!("Enter configurator");
        Ok(())
    }

    async fn reset_fc(&mut self) -> Result<(), super::DroneCommunicationError> {
        self.status = FcStatus {
            valid: true,
            total_duration: 30f32.into(),
            inner_duration: 40f32.into(),
            min_measured_period: 1001.into(),
            max_measured_period: 1002.into(),
            fc_phase: FcPhase::Armed,
        };
        info!("Reset");
        Ok(())
    }

    async fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: common::shared_objects::MotorDirection,
    ) -> Result<(), super::DroneCommunicationError> {
        info!("Driving {motor_index} {direction}");
        Ok(())
    }
}
