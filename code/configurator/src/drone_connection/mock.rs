use common::{
    configurator_protocol::messages::{BatteryResponse, FcPhase, FcStatus},
    shared_objects::StoredConfig,
};

use crate::drone_connection::DroneConnection;

pub struct MockDroneConnection {
    config: StoredConfig,
    status: FcStatus,
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

impl DroneConnection for MockDroneConnection {
    fn test_connection(&mut self) -> bool {
        true
    }

    fn read_config(
        &mut self,
    ) -> Result<common::shared_objects::StoredConfig, super::DroneCommunicationError> {
        Ok(self.config.clone())
    }

    fn save_config(
        &mut self,
        mut config: common::shared_objects::StoredConfig,
    ) -> Result<(), super::DroneCommunicationError> {
        config.update_checksum();
        self.config = config;

        Ok(())
    }

    fn query_motor_count(&mut self) -> Result<u8, super::DroneCommunicationError> {
        Ok(4)
    }

    fn query_status(
        &mut self,
    ) -> Result<common::configurator_protocol::messages::FcStatus, super::DroneCommunicationError>
    {
        Ok(self.status)
    }

    fn query_battery(
        &mut self,
    ) -> Result<
        common::configurator_protocol::messages::BatteryResponse,
        super::DroneCommunicationError,
    > {
        Ok(BatteryResponse {
            voltage: 15f32.into(),
            cell_count: 4,
        })
    }

    fn enter_configurator(&mut self) -> Result<(), super::DroneCommunicationError> {
        self.status.fc_phase = FcPhase::Config;

        Ok(())
    }

    fn reset_fc(&mut self) -> Result<(), super::DroneCommunicationError> {
        self.status = FcStatus {
            valid: true,
            total_duration: 30f32.into(),
            inner_duration: 40f32.into(),
            min_measured_period: 1001.into(),
            max_measured_period: 1002.into(),
            fc_phase: FcPhase::Armed,
        };

        Ok(())
    }

    fn drive_motor(
        &mut self,
        motor_index: u8,
        direction: common::shared_objects::MotorDirection,
    ) -> Result<(), super::DroneCommunicationError> {
        println!("Driving {motor_index} {direction}");

        Ok(())
    }
}
