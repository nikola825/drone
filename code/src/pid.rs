use core::f32;

use embassy_time::Instant;

use crate::{
    ahrs_wrapper::AhrsWrapper,
    config_storage::StoredConfig,
    crsf::CRSFChannels,
    icm42688::ICM42688,
    math_stuff::{angle_add, angle_sub},
    motors::MotorInputs,
};

// At our PID rate of 1000Hz, this gives a 3dB dropoff at around 100Hz
const D_TERM_LPF_FACTOR: f32 = 0.457f32;

struct Pid {
    i: f32,
    last_error: f32,
    d_term_lpf: f32,
}

impl Pid {
    fn new() -> Self {
        Pid {
            i: 0f32,
            last_error: 0f32,
            d_term_lpf: 0f32,
        }
    }

    fn calculate(&mut self, kp: f32, ki: f32, kd: f32, error: f32, dt: f32, i_limit: f32) -> f32 {
        self.i += error * dt * ki;

        self.i = self.i.clamp(-i_limit, i_limit);

        // Do basic exponential low-pass on the D term
        let d_term = (error - self.last_error) * kd / dt;
        self.d_term_lpf = self.d_term_lpf * (1f32 - D_TERM_LPF_FACTOR) + d_term * D_TERM_LPF_FACTOR;

        let result = kp * error + self.i + self.d_term_lpf;
        self.last_error = error;

        result
    }

    fn reset(&mut self) {
        self.last_error = 0f32;
        self.i = 0f32;
        self.d_term_lpf = 0f32;
    }
}

pub struct PidContext {
    yaw_pid: Pid,
    pitch_pid: Pid,
    roll_pid: Pid,

    yaw_offset: f32,
    pitch_offset: f32,
    roll_offset: f32,

    last_pid_time: Instant,

    pub orient: AhrsWrapper,

    current_yaw_angle: f32,
}

impl PidContext {
    pub fn new(stored_config: &StoredConfig) -> Self {
        PidContext {
            yaw_pid: Pid::new(),
            pitch_pid: Pid::new(),
            roll_pid: Pid::new(),
            yaw_offset: stored_config.yaw_offset.into(),
            pitch_offset: stored_config.pitch_offset.into(),
            roll_offset: stored_config.roll_offset.into(),
            last_pid_time: Instant::now(),
            orient: AhrsWrapper::new(),
            current_yaw_angle: 0f32,
        }
    }

    fn reset(&mut self, current_yaw_angle: f32) {
        self.yaw_pid.reset();
        self.pitch_pid.reset();
        self.roll_pid.reset();
        self.last_pid_time = Instant::now();
        self.current_yaw_angle = current_yaw_angle;
    }
}

fn angle_mode_target_angular_velocity(angle: f32, target_angle: f32, velocity_scale: f32) -> f32 {
    let angle_error = angle_sub(target_angle, angle);
    let mut target_velocity = angle_error * velocity_scale;

    if angle_error < -0.1 {
        target_velocity -= 1.0f32;
    }
    if angle_error > 0.1 {
        target_velocity += 1.0f32;
    }

    target_velocity
}

pub async fn do_pid_iteration(
    imu: &mut ICM42688,
    context: &mut PidContext,
    inputs: &CRSFChannels,
) -> MotorInputs {
    let motion_data = imu.get_motion_data().apply_gyro_offsets(
        context.yaw_offset,
        context.pitch_offset,
        context.roll_offset,
    );

    context.orient.update(&motion_data);

    let euler = context.orient.read_ypr();

    let angle_mode = inputs.mode() > 0;

    let (yaw_target_angular_velocity, pitch_target_angular_velocity, roll_target_angular_velocity) =
        if angle_mode {
            let yaw_target_velocity = if inputs.yaw_expo().abs() > 5f32 {
                context.current_yaw_angle = euler.yaw;

                inputs.yaw_expo()
            } else {
                angle_mode_target_angular_velocity(
                    euler.yaw,
                    angle_add(context.current_yaw_angle, inputs.yaw_angle()),
                    inputs.aux1(),
                )
            };

            (
                yaw_target_velocity,
                angle_mode_target_angular_velocity(
                    euler.pitch,
                    inputs.pitch_angle(),
                    inputs.aux1(),
                ),
                angle_mode_target_angular_velocity(euler.roll, inputs.roll_angle(), inputs.aux1()),
            )
        } else {
            (inputs.yaw_expo(), inputs.pitch_expo(), inputs.roll_expo())
        };

    let yaw_error = yaw_target_angular_velocity - motion_data.gyro_yaw;
    let pitch_error = pitch_target_angular_velocity - motion_data.gyro_pitch;
    let roll_error = roll_target_angular_velocity - motion_data.gyro_roll;

    let thrust = inputs.throttle() as f32;
    let command_limit = thrust / 4f32;

    let motor_thrust = inputs.throttle() as u32;

    let thrust_scaled = (motor_thrust * (100 - inputs.aux2() as u32)) / 100;

    let motor_thrust = thrust_scaled as u16;

    let now = Instant::now();
    let dt = (now - context.last_pid_time).as_micros() as f32 * 1e-6;
    context.last_pid_time = now;

    if motor_thrust < 200 {
        context.reset(euler.yaw);

        MotorInputs::idle(motor_thrust)
    } else {
        let yaw_input = context
            .yaw_pid
            .calculate(100f32, 72f32, 0f32, yaw_error, dt, command_limit)
            .clamp(-command_limit, command_limit) as i16;

        let pitch_input = context
            .pitch_pid
            .calculate(12.6f32, 60f32, 0.045f32, pitch_error, dt, command_limit)
            .clamp(-command_limit, command_limit) as i16;

        let roll_input = context
            .roll_pid
            .calculate(12.6f32, 60f32, 0.045f32, roll_error, dt, command_limit)
            .clamp(-command_limit, command_limit) as i16;

        MotorInputs {
            motor_thrust,
            yaw_input,
            pitch_input,
            roll_input,
        }
    }
}
