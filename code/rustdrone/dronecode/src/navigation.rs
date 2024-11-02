use embassy_time::Instant;

use crate::{crsf::CRSFChannels, icm42688::ICM42688, DroneContext};

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
    pub motor_thrust: u16,

    pub yaw_input: i16,
    pub roll_input: i16,
    pub pitch_input: i16,

    yaw_pid: Pid,
    pitch_pid: Pid,
    roll_pid: Pid,

    last_pid_time: Instant
}

impl NavigationContext {
    pub fn new() -> Self {
        NavigationContext {
            motor_thrust: 0,
            yaw_input: 0,
            roll_input: 0,
            pitch_input: 0,

            yaw_pid: Pid::new(),
            pitch_pid: Pid::new(),
            roll_pid: Pid::new(),
            last_pid_time: Instant::now()
        }
    }

    fn zero(&mut self) {
        self.yaw_input = 0;
        self.pitch_input = 0;
        self.roll_input = 0;

        self.yaw_pid.reset();
        self.pitch_pid.reset();
        self.roll_pid.reset();
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

pub fn navigate(imu: &mut ICM42688, context: &mut DroneContext, inputs: &CRSFChannels) {
    const YAW_OFFSET:f32 = 0.47141057;
    const PITCH_OFFSET:f32 = 0.3117653;
    const ROLL_OFFSET:f32 = 0.088096835;
    let (yaw_measured, pitch_measured, roll_measured) = imu.get_ypr_deg();

    let yaw_measured = yaw_measured - YAW_OFFSET;
    let pitch_measured = pitch_measured - PITCH_OFFSET;
    let roll_measured = roll_measured - ROLL_OFFSET;

    let roll_measured = roll_measured * -1f32;

    let yaw_error = (inputs.yaw() as f32) - yaw_measured;
    let pitch_error = (inputs.pitch() as f32) - pitch_measured;
    let roll_error = (inputs.roll() as f32) - roll_measured;

    let context = &mut context.navigation_context;
    let thrust = inputs.throttle() as f32;
    let command_limit = thrust / 4f32;

    context.motor_thrust = inputs.throttle();

    let now = Instant::now();
    let dt = (now - context.last_pid_time).as_micros() as f32 * 1e-6;
    context.last_pid_time = now;

    let aux = inputs.aux() as f32;
    let scale_factor = if aux >= 64f32 {
        1f32 + (aux - 64f32) / 64f32
    } else {
        1f32 - (64f32 - aux) / 128f32
    };

    if context.motor_thrust < 200 {
        context.zero();
    } else {
        let yaw_input =
            context
                .yaw_pid
                .calculate(100f32, 1231 as f32, 0f32, yaw_error, dt, command_limit);
        context.yaw_input = range_limit(yaw_input, command_limit) as i16;

        let pitch_input = context.pitch_pid.calculate(
            12.6f32 * scale_factor,
            60f32 * scale_factor,
            0f32,
            pitch_error,
            dt,
            command_limit,
        );
        context.pitch_input = range_limit(pitch_input, command_limit) as i16;

        let roll_input = context.roll_pid.calculate(
            12.6f32 * scale_factor,
            60f32,
            0f32 * scale_factor,
            roll_error,
            dt,
            command_limit,
        );
        context.roll_input = range_limit(roll_input, command_limit) as i16;
    }
}
