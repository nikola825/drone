use common::configurator_protocol::messages::{FcPhase, FcStatus, MotorOverride};
use defmt::info;
use embassy_time::{Duration, Instant, Ticker};

use crate::{
    hal::{Leds, ESC_COUNT},
    motor::{esc_dshot::DshotCommand, Motor},
    shared_state::SharedState,
};

enum MotorCommandOverride {
    None,
    Initial(usize, DshotCommand, u8),
    SetDirection(usize, DshotCommand, u8),
    SaveSetting(usize, u8),
    Rest(usize, u8),
    Drive(usize, u16),
}

impl MotorCommandOverride {
    pub fn next(self) -> Self {
        match self {
            MotorCommandOverride::Initial(motor, dir, rem) => {
                if rem > 0 {
                    Self::Initial(motor, dir, rem - 1)
                } else {
                    Self::SetDirection(motor, dir, 10)
                }
            }
            MotorCommandOverride::SetDirection(motor, dir, rem) => {
                if rem > 0 {
                    Self::SetDirection(motor, dir, rem - 1)
                } else {
                    Self::SaveSetting(motor, 35)
                }
            }
            MotorCommandOverride::SaveSetting(motor, rem) => {
                if rem > 0 {
                    Self::SaveSetting(motor, rem - 1)
                } else {
                    Self::Rest(motor, 35)
                }
            }
            MotorCommandOverride::Rest(motor, rem) => {
                if rem > 0 {
                    Self::Rest(motor, rem - 1)
                } else {
                    Self::Drive(motor, 10000)
                }
            }
            MotorCommandOverride::Drive(motor, rem) => {
                if rem > 0 {
                    Self::Drive(motor, rem - 1)
                } else {
                    Self::None
                }
            }
            MotorCommandOverride::None => MotorCommandOverride::None,
        }
    }
}

impl From<MotorOverride> for MotorCommandOverride {
    fn from(motor_override: MotorOverride) -> Self {
        let direction = match motor_override.direction {
            common::configurator_protocol::messages::MotorOverrideDirection::Forward => {
                DshotCommand::DSHOT_CMD_SPIN_DIRECTION_1
            }
            common::configurator_protocol::messages::MotorOverrideDirection::Backward => {
                DshotCommand::DSHOT_CMD_SPIN_DIRECTION_2
            }
        };

        Self::Initial(motor_override.motor_index as usize, direction, 35)
    }
}

impl From<Option<MotorOverride>> for MotorCommandOverride {
    fn from(motor_override: Option<MotorOverride>) -> Self {
        match motor_override {
            Some(motor_override) => motor_override.into(),
            None => Self::None,
        }
    }
}

pub async fn configurator_loop(
    motors: [Motor; ESC_COUNT],
    shared_state: &'static SharedState,
    mut leds: Leds,
) -> ! {
    const PID_PERIOD_US: u64 = 1005;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    leds.green_off();
    leds.blue_off();
    leds.yellow_on();

    let mut total_duration = 0f32;
    let mut inner_duration = 0f32;
    let mut previous_t1 = Instant::now();
    let mut max_measured_period = 0;
    let mut min_measured_period = 10000;

    let mut motor_override = MotorCommandOverride::None;

    loop {
        let t1 = Instant::now();

        let t2 = Instant::now();
        print_counter += 1;

        let mut motor_commands = [0u16; ESC_COUNT];

        if matches!(motor_override, MotorCommandOverride::None) {
            motor_override = shared_state.query_motor_override().into();
        } else {
            match &motor_override {
                MotorCommandOverride::None => {}
                MotorCommandOverride::Initial(_, _, _) => {}
                MotorCommandOverride::SetDirection(motor_index, dshot_command, _) => {
                    motor_commands[*motor_index] = *dshot_command as u16;
                }
                MotorCommandOverride::SaveSetting(motor_index, _) => {
                    motor_commands[*motor_index] = DshotCommand::DSHOT_CMD_SAVE_SETTINGS as u16;
                }
                MotorCommandOverride::Rest(_, _) => {}
                MotorCommandOverride::Drive(motor_index, _) => {
                    motor_commands[*motor_index] = 80;
                }
            }

            motor_override = motor_override.next();
        }

        Motor::multi_send(motors.each_ref(), motor_commands);

        let t3 = Instant::now();

        total_duration = (t3 - t1).as_micros() as f32 * 0.5 + total_duration * 0.5;
        inner_duration = (t3 - t2).as_micros() as f32 * 0.5 + inner_duration * 0.5;
        let measured_period: u64 = (t1 - previous_t1).as_micros();
        if measured_period > max_measured_period {
            max_measured_period = measured_period;
        }
        if measured_period < min_measured_period {
            min_measured_period = measured_period;
        }

        if print_counter > 1000 * (2000 / PID_PERIOD_US) {
            print_counter = 0;
            info!(
                "TICK {} {} {} {}",
                total_duration, inner_duration, min_measured_period, max_measured_period,
            );
            shared_state.publish_status(FcStatus {
                inner_duration: inner_duration.into(),
                total_duration: total_duration.into(),
                min_measured_period: min_measured_period.into(),
                max_measured_period: max_measured_period.into(),
                fc_phase: FcPhase::Config,
                valid: true,
            });
            max_measured_period = 0;
            min_measured_period = 10000;
        }
        previous_t1 = t1;
        ticker.next().await;
    }
}
