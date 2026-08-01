use core::fmt::Display;

use embassy_time::{Duration, Instant, Timer};

use crate::{mixer::MotorMix, motor::esc_dshot::BeepTone};

pub struct MotorsContext {
    motors: MotorMix,
    running: bool,
    beep_interval_start: Instant,
    beep_tone: BeepTone,
}

impl MotorsContext {
    pub fn new(motors: MotorMix) -> Self {
        MotorsContext {
            motors,
            running: false,
            beep_interval_start: Instant::MIN,
            beep_tone: BeepTone::Tone1,
        }
    }

    pub fn into_mix(self) -> MotorMix {
        self.motors
    }
}

pub struct MotorInputs {
    pub motor_thrust: u16,
    pub yaw_input: i16,
    pub roll_input: i16,
    pub pitch_input: i16,

    pub servo_roll: i16,
    pub servo_pitch: i16,
}

impl MotorInputs {
    pub fn idle(motor_thrust: u16, servo_roll: i16, servo_pitch: i16) -> Self {
        MotorInputs {
            motor_thrust,
            yaw_input: 0,
            roll_input: 0,
            pitch_input: 0,
            servo_roll,
            servo_pitch,
        }
    }
}

impl Default for MotorInputs {
    fn default() -> Self {
        Self::idle(0, 0, 0)
    }
}

impl Display for MotorInputs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "sr: {}; sp: {}", self.servo_roll, self.servo_pitch)
    }
}

async fn gentle_stop(current_thrust: u16, context: &mut MotorsContext) {
    let mut thrust_target = current_thrust;

    while thrust_target > 200 {
        context.motors.same_throttle(thrust_target);

        Timer::after_millis(100).await;

        thrust_target = thrust_target * 70 / 100;
    }

    zero_throttle(context);
    context.running = false;
}

fn zero_throttle(context: &MotorsContext) {
    context.motors.zero_throttle();
}

fn beep_motors(context: &mut MotorsContext) {
    const BEEP_INTERVAL: Duration = Duration::from_millis(500);
    const BEEP_DUTY: Duration = Duration::from_millis(100);

    let now = Instant::now();
    let delta_t = now - context.beep_interval_start;

    if delta_t > BEEP_INTERVAL {
        context.beep_tone = context.beep_tone.next();
        context.beep_interval_start = now;
        zero_throttle(context);
    } else if delta_t > BEEP_DUTY {
        zero_throttle(context);
    } else {
        context.motors.beep_escs(context.beep_tone);
    }
}

pub async fn disarm(context: &mut MotorsContext, inputs: &MotorInputs, beep: bool) {
    if context.running {
        gentle_stop(
            inputs.motor_thrust / context.motors.esc_motor_couint(),
            context,
        )
        .await;
    } else if beep {
        beep_motors(context);
    } else {
        zero_throttle(context);
    }

    center_servos(context);
}

pub fn drive_motors(context: &mut MotorsContext, inputs: &MotorInputs) {
    context.running = true;
    if inputs.motor_thrust > 0 {
        context.motors.drive_escs(inputs);
    } else {
        zero_throttle(context);
    }

    context.motors.drive_servos(inputs);
}

pub fn center_servos(context: &mut MotorsContext) {
    context.motors.center_servos();
}
