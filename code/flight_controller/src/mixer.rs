use common::shared_objects::{MotorPosition, StoredConfig};

use crate::{
    esc::{motor_control::BeepTone, EscMotorSet},
    hal::{ServoDriver, ESC_COUNT},
    motors::MotorInputs,
};

#[derive(Default)]
struct MixCoefficients {
    thrust: i16,
    yaw: i16,
    pitch: i16,
    roll: i16,
    servo_roll: i16,
    servo_pitch: i16,
}

#[derive(Default)]
struct ThrustMotor {
    motor_index: usize,
    coefficients: MixCoefficients,
}

struct ServoMotor {
    motor: ServoDriver,
    coefficients: MixCoefficients,
}

pub struct GenericMotorMix<const SERVO_COUNT: usize> {
    motor_set: EscMotorSet,
    esc_motors: [ThrustMotor; ESC_COUNT],
    esc_coefficient_divider: i16,
    servos: [ServoMotor; SERVO_COUNT],
}

#[allow(dead_code)]
pub type QuadcopterMix = GenericMotorMix<0>;
#[allow(dead_code)]
pub type WingMix = GenericMotorMix<2>;

#[cfg(feature = "quad")]
impl QuadcopterMix {
    #[allow(dead_code)]
    pub fn new(motor_set: EscMotorSet, config: &StoredConfig) -> Self {
        let mut esc_motors: [ThrustMotor; 4] = Default::default();

        for (index, motor) in esc_motors.iter_mut().enumerate() {
            motor.motor_index = index;
        }

        esc_motors[config.motor_positions[MotorPosition::FrontLeft] as usize].coefficients =
            MixCoefficients {
                thrust: 1,
                roll: 1,
                pitch: -1,
                yaw: -1,
                ..Default::default()
            };

        esc_motors[config.motor_positions[MotorPosition::FrontRight] as usize].coefficients =
            MixCoefficients {
                thrust: 1,
                roll: -1,
                pitch: -1,
                yaw: 1,
                ..Default::default()
            };

        esc_motors[config.motor_positions[MotorPosition::RearLeft] as usize].coefficients =
            MixCoefficients {
                thrust: 1,
                roll: 1,
                pitch: 1,
                yaw: 1,
                ..Default::default()
            };

        esc_motors[config.motor_positions[MotorPosition::RearRight] as usize].coefficients =
            MixCoefficients {
                thrust: 1,
                roll: -1,
                pitch: 1,
                yaw: -1,
                ..Default::default()
            };

        Self {
            esc_motors,
            motor_set,
            servos: [],
            esc_coefficient_divider: 4,
        }
    }

    pub fn into_motors(self) -> EscMotorSet {
        self.motor_set
    }
}

#[cfg(feature = "wing")]
impl WingMix {
    #[allow(dead_code)]
    pub fn new(
        left_servo: ServoDriver,
        right_servo: ServoDriver,
        mut thrust_motor: EscMotorSet<1>,
    ) -> Self {
        thrust_motor.enter_dshot_mode();

        Self {
            esc_motors: [ThrustMotor {
                motor_index: 0,
                coefficients: MixCoefficients {
                    thrust: 3,
                    ..Default::default()
                },
            }],
            motor_set: thrust_motor,
            esc_coefficient_divider: 10,
            servos: [
                ServoMotor {
                    motor: left_servo,
                    coefficients: MixCoefficients {
                        servo_roll: -1,
                        servo_pitch: -1,
                        ..Default::default()
                    },
                },
                ServoMotor {
                    motor: right_servo,
                    coefficients: MixCoefficients {
                        servo_roll: 1,
                        servo_pitch: -1,
                        ..Default::default()
                    },
                },
            ],
        }
    }

    #[allow(dead_code)]
    pub fn into_motors(self) -> EscMotorSet<1> {
        self.motor_set
    }
}

impl<const SERVO_COUNT: usize> GenericMotorMix<SERVO_COUNT> {
    pub async fn drive_escs(&mut self, inputs: &MotorInputs) {
        let thrust = inputs.motor_thrust as i16;
        let yaw_input = inputs.yaw_input;
        let pitch_input = inputs.pitch_input;
        let roll_input = inputs.roll_input;

        let mut outputs: [u16; ESC_COUNT] = [0; ESC_COUNT];

        for (index, motor) in self.esc_motors.iter().enumerate() {
            outputs[index] = ((thrust * motor.coefficients.thrust
                + yaw_input * motor.coefficients.yaw
                + pitch_input * motor.coefficients.pitch
                + roll_input * motor.coefficients.roll)
                / self.esc_coefficient_divider)
                .clamp(0, 2047) as u16;
        }

        self.motor_set.multi_throttle(outputs).await;
    }

    pub fn drive_servos(&mut self, inputs: &MotorInputs) {
        for servo in &mut self.servos {
            let angle = (inputs.servo_pitch * servo.coefficients.servo_pitch
                + inputs.servo_roll * servo.coefficients.servo_roll)
                .clamp(-45, 45);

            servo.motor.update(angle);
        }
    }

    pub fn center_servos(&mut self) {
        for servo in &mut self.servos {
            servo.motor.update(0);
        }
    }

    pub async fn zero_throttle(&mut self) {
        self.motor_set.multi_throttle([0; ESC_COUNT]).await;
    }

    pub async fn beep_escs(&mut self, tone: BeepTone) {
        self.motor_set.beep(tone).await
    }

    pub async fn same_throttle(&mut self, throttle: u16) {
        self.motor_set.multi_throttle([throttle; ESC_COUNT]).await;
    }

    pub fn esc_motor_couint(&self) -> u16 {
        self.esc_motors.len() as u16
    }
}

#[cfg(feature = "wing")]
pub type MotorMix = WingMix;

#[cfg(feature = "quad")]
pub type MotorMix = QuadcopterMix;
