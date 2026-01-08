use crate::{
    hal::ServoDriver,
    motors::{BeepTone, Motor, MotorInputs},
};

#[derive(Default)]
struct MixCoefficients {
    thrust: i16,
    yaw: i16,
    pitch: i16,
    roll: i16,
    left_aileron: i16,
    right_aileron: i16,
    elevator: i16,
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
type QuadcopterMix = GenericMotorMix<4, 0>;
#[allow(dead_code)]
type WingMix = GenericMotorMix<1, 2>;

impl QuadcopterMix {
    #[allow(dead_code)]
    pub fn new(front_left: Motor, front_right: Motor, rear_left: Motor, rear_right: Motor) -> Self {
        Self {
            esc_motors: [
                EscMotor {
                    motor: front_left,
                    coefficients: MixCoefficients {
                        thrust: 1,
                        roll: 1,
                        pitch: -1,
                        yaw: -1,
                        ..Default::default()
                    },
                },
                EscMotor {
                    motor: front_right,
                    coefficients: MixCoefficients {
                        thrust: 1,
                        roll: -1,
                        pitch: -1,
                        yaw: 1,
                        ..Default::default()
                    },
                },
                EscMotor {
                    motor: rear_left,
                    coefficients: MixCoefficients {
                        thrust: 1,
                        roll: 1,
                        pitch: 1,
                        yaw: 1,
                        ..Default::default()
                    },
                },
                EscMotor {
                    motor: rear_right,
                    coefficients: MixCoefficients {
                        thrust: 1,
                        roll: -1,
                        pitch: 1,
                        yaw: -1,
                        ..Default::default()
                    },
                },
            ],
            servos: [],
            esc_coefficient_divider: 4,
        }
    }
}

impl WingMix {
    #[allow(dead_code)]
    pub fn new(left_servo: ServoDriver, right_servo: ServoDriver, thrust_motor: Motor) -> Self {
        Self {
            esc_motors: [EscMotor {
                motor: thrust_motor,
                coefficients: MixCoefficients {
                    thrust: 1,
                    ..Default::default()
                },
            }],
            esc_coefficient_divider: 5,
            servos: [
                ServoMotor {
                    motor: left_servo,
                    coefficients: MixCoefficients {
                        left_aileron: 1,
                        right_aileron: 1,
                        elevator: 1,
                        ..Default::default()
                    },
                },
                ServoMotor {
                    motor: right_servo,
                    coefficients: MixCoefficients {
                        left_aileron: 1,
                        right_aileron: 1,
                        elevator: 1,
                        ..Default::default()
                    },
                },
            ],
        }
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
            let angle = (inputs.elevator * servo.coefficients.elevator
                + inputs.left_aileron * servo.coefficients.left_aileron
                + inputs.right_aileron * servo.coefficients.right_aileron)
                .clamp(-90, 90);

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
