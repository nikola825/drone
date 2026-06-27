use common::shared_objects::StoredConfig;

use crate::{
    hal::ServoDriver,
    motor::{esc_dshot::BeepTone, Motor},
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

struct EscMotor {
    motor: Motor,
    coefficients: MixCoefficients,
}

struct ServoMotor {
    motor: ServoDriver,
    coefficients: MixCoefficients,
}

pub struct GenericMotorMix<const ESC_COUNT: usize, const SERVO_COUNT: usize> {
    esc_motors: [EscMotor; ESC_COUNT],
    esc_coefficient_divider: i16,
    servos: [ServoMotor; SERVO_COUNT],
}

#[allow(dead_code)]
pub type QuadcopterMix = GenericMotorMix<4, 0>;
#[allow(dead_code)]
pub type WingMix = GenericMotorMix<1, 2>;

impl QuadcopterMix {
    #[allow(dead_code)]
    pub fn new(motors: [Motor; 4], config: &StoredConfig) -> Self {
        let mut esc_motors = motors.map(|motor| EscMotor {
            motor,
            coefficients: MixCoefficients::default(),
        });

        for esc_motor in &mut esc_motors {
            esc_motor.motor.enter_dshot_mode();
        }

        esc_motors[config.front_left_motor as usize].coefficients = MixCoefficients {
            thrust: 1,
            roll: 1,
            pitch: -1,
            yaw: -1,
            ..Default::default()
        };

        esc_motors[config.front_right_motor as usize].coefficients = MixCoefficients {
            thrust: 1,
            roll: -1,
            pitch: -1,
            yaw: 1,
            ..Default::default()
        };

        esc_motors[config.rear_left_motor as usize].coefficients = MixCoefficients {
            thrust: 1,
            roll: 1,
            pitch: 1,
            yaw: 1,
            ..Default::default()
        };

        esc_motors[config.rear_right_motor as usize].coefficients = MixCoefficients {
            thrust: 1,
            roll: -1,
            pitch: 1,
            yaw: -1,
            ..Default::default()
        };

        Self {
            esc_motors,
            servos: [],
            esc_coefficient_divider: 4,
        }
    }

    pub fn into_motors(self) -> [Motor; 4] {
        self.esc_motors.map(|esc_motor| esc_motor.motor)
    }
}

impl WingMix {
    #[allow(dead_code)]
    pub fn new(left_servo: ServoDriver, right_servo: ServoDriver, thrust_motor: Motor) -> Self {
        Self {
            esc_motors: [EscMotor {
                motor: thrust_motor,
                coefficients: MixCoefficients {
                    thrust: 3,
                    ..Default::default()
                },
            }],
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
    pub fn into_motors(self) -> [Motor; 1] {
        self.esc_motors.map(|esc_motor| esc_motor.motor)
    }
}

impl<const ESC_COUNT: usize, const SERVO_COUNT: usize> GenericMotorMix<ESC_COUNT, SERVO_COUNT> {
    pub fn drive_escs(&self, inputs: &MotorInputs) {
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

        Motor::multi_throttle(
            self.esc_motors.each_ref().map(|motor| &motor.motor),
            outputs,
        );
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

    pub fn zero_throttle(&self) {
        Motor::multi_throttle(
            self.esc_motors.each_ref().map(|motor| &motor.motor),
            [0; ESC_COUNT],
        );
    }

    pub fn beep_escs(&self, tone: BeepTone) {
        self.esc_motors.iter().for_each(|motor| {
            motor.motor.beep(tone);
        });
    }

    pub fn same_throttle(&self, throttle: u16) {
        Motor::multi_throttle(
            self.esc_motors.each_ref().map(|motor| &motor.motor),
            [throttle; ESC_COUNT],
        );
    }

    pub fn esc_motor_couint(&self) -> u16 {
        self.esc_motors.len() as u16
    }
}

#[cfg(feature = "wing")]
pub type MotorMix = WingMix;

#[cfg(feature = "quad")]
pub type MotorMix = QuadcopterMix;
