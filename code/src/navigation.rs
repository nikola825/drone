use embassy_time::Instant;

use crate::{crsf::CRSFChannels, icm42688::ICM42688, motors::MotorInputs};

struct Pid {
    i: f32,
    last_error: f32,
}

impl Pid {
    fn new() -> Self {
        Pid {
            i: 0f32,
            last_error: 0f32,
        }
    }

    fn calculate(&mut self, kp: f32, ki: f32, kd: f32, error: f32, dt: f32, i_limit: f32) -> f32 {
        self.i += error * dt * ki;

        self.i = if self.i > i_limit {
            i_limit
        } else if self.i < -i_limit {
            -i_limit
        } else {
            self.i
        };

        let result = kp * error + self.i + (error - self.last_error) * kd / dt;
        self.last_error = error;

        return result;
    }

    fn reset(&mut self) {
        self.last_error = 0f32;
        self.i = 0f32;
    }
}

pub struct NavigationContext {
    yaw_pid: Pid,
    pitch_pid: Pid,
    roll_pid: Pid,

    last_pid_time: Instant,
}

impl NavigationContext {
    pub fn new() -> Self {
        NavigationContext {
            yaw_pid: Pid::new(),
            pitch_pid: Pid::new(),
            roll_pid: Pid::new(),
            last_pid_time: Instant::now(),
        }
    }

    fn reset(&mut self) {
        self.yaw_pid.reset();
        self.pitch_pid.reset();
        self.roll_pid.reset();
        self.last_pid_time = Instant::now();
    }
}

fn range_limit(value: f32, limit: f32) -> f32 {
    if value > limit {
        limit
    } else if value < -limit {
        -limit
    } else {
        value
    }
}

pub fn navigate(
    imu: &mut ICM42688,
    context: &mut NavigationContext,
    inputs: &CRSFChannels,
) -> MotorInputs {
    const YAW_OFFSET: f32 = 0.47141057;
    const PITCH_OFFSET: f32 = 0.3117653;
    const ROLL_OFFSET: f32 = 0.088096835;
    let (yaw_measured, pitch_measured, roll_measured) = imu.get_ypr_deg();

    let yaw_measured = yaw_measured - YAW_OFFSET;
    let pitch_measured = pitch_measured - PITCH_OFFSET;
    let roll_measured = roll_measured - ROLL_OFFSET;

    let roll_measured = roll_measured * -1f32;

    let yaw_error = (inputs.yaw() as f32) - yaw_measured;
    let pitch_error = (inputs.pitch() as f32) - pitch_measured;
    let roll_error = (inputs.roll() as f32) - roll_measured;

    let thrust = inputs.throttle() as f32;
    let command_limit = thrust / 4f32;

    let motor_thrust = inputs.throttle();

    let now = Instant::now();
    let dt = (now - context.last_pid_time).as_micros() as f32 * 1e-6;
    context.last_pid_time = now;

    let p_scale_factor = 1.0f32 + (inputs.aux1() / 2) as f32 / 8f32;

    let d = (inputs.aux2() / 2) as f32 / 2.0f32;

    if motor_thrust < 200 {
        context.reset();

        return MotorInputs::idle(motor_thrust);
    } else {
        let yaw_input =
            context
                .yaw_pid
                .calculate(100f32, 72f32, 0f32, yaw_error, dt, command_limit);

        let yaw_input = range_limit(yaw_input, command_limit) as i16;

        let pitch_input = context.pitch_pid.calculate(
            12.6f32 * p_scale_factor,
            60f32,
            d,
            pitch_error,
            dt,
            command_limit,
        );
        let pitch_input = range_limit(pitch_input, command_limit) as i16;

        let roll_input = context.roll_pid.calculate(
            12.6f32 * p_scale_factor,
            60f32,
            d,
            roll_error,
            dt,
            command_limit,
        );
        let roll_input = range_limit(roll_input, command_limit) as i16;

        return MotorInputs {
            motor_thrust: motor_thrust,
            yaw_input: yaw_input,
            pitch_input: pitch_input,
            roll_input: roll_input,
        };
    }
}
