use common::configurator_protocol::messages::MotorDirectionSetting;
use embassy_time::{Duration, Instant};

use crate::esc::motor_control::DshotCommand;

enum StateMachineState {
    Initial(u16),
    ChangeSetting(u16),
    SaveSetting(u16),
    Rest(u16),
    Drive,
}

pub struct MotorSettingStateMachine {
    setting1: DshotCommand,
    setting2: Option<DshotCommand>,
    motor_index: u8,
    state: StateMachineState,
    drive_for: Duration,
    drive_start: Instant,
}

impl MotorSettingStateMachine {
    fn new(
        setting1: DshotCommand,
        setting2: Option<DshotCommand>,
        motor_index: u8,
        drive_for: Duration,
    ) -> Self {
        Self {
            setting1,
            setting2,
            motor_index,
            state: StateMachineState::Initial(35),
            drive_for,
            drive_start: Instant::now(),
        }
    }

    pub fn motor_index(&self) -> u8 {
        self.motor_index
    }

    pub fn advance(&mut self) -> Option<DshotCommand> {
        match &self.state {
            StateMachineState::Initial(rem) => {
                if *rem == 0 {
                    self.state = StateMachineState::ChangeSetting(10);
                } else {
                    self.state = StateMachineState::Initial(*rem - 1);
                }
                Some(DshotCommand::DSHOT_CMD_STOP)
            }
            StateMachineState::ChangeSetting(rem) => {
                if *rem == 0 {
                    self.state = StateMachineState::SaveSetting(35);
                } else {
                    self.state = StateMachineState::ChangeSetting(*rem - 1);
                }
                Some(self.setting1)
            }
            StateMachineState::SaveSetting(rem) => {
                if *rem == 0 {
                    self.state = StateMachineState::Rest(35);
                } else {
                    self.state = StateMachineState::SaveSetting(*rem - 1);
                }
                Some(DshotCommand::DSHOT_CMD_SAVE_SETTINGS)
            }
            StateMachineState::Rest(rem) => {
                if *rem == 0 {
                    self.state = StateMachineState::Drive;
                    self.drive_start = Instant::now();
                } else {
                    self.state = StateMachineState::Rest(*rem - 1);
                }
                Some(DshotCommand::DSHOT_CMD_STOP)
            }
            StateMachineState::Drive => {
                if let Some(setting2) = self.setting2 {
                    self.setting1 = setting2;
                    self.setting2 = None;
                    self.state = StateMachineState::Initial(35);
                    Some(DshotCommand::DSHOT_CMD_STOP)
                } else if self.drive_start + self.drive_for > Instant::now() {
                    Some(DshotCommand::DSHOT_CMD_CUSTOM_DRIVE_SLOW_FOR_TEST)
                } else {
                    None
                }
            }
        }
    }
}

impl From<MotorDirectionSetting> for MotorSettingStateMachine {
    fn from(value: MotorDirectionSetting) -> Self {
        let direction_command = match value.direction {
            common::shared_objects::MotorDirection::Forward => {
                DshotCommand::DSHOT_CMD_SPIN_DIRECTION_1
            }
            common::shared_objects::MotorDirection::Backward => {
                DshotCommand::DSHOT_CMD_SPIN_DIRECTION_2
            }
        };

        Self::new(
            DshotCommand::DSHOT_CMD_3D_MODE_OFF,
            Some(direction_command),
            value.motor_index,
            Duration::from_secs(10),
        )
    }
}
