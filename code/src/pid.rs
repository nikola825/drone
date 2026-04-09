use core::f32;

use embassy_time::Instant;

use crate::{
    crsf::CRSFChannels,
    flight_control::PidInputs,
    model::{PITCH_KD, PITCH_KI, PITCH_KP, ROLL_KD, ROLL_KI, ROLL_KP, YAW_KD, YAW_KI, YAW_KP},
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

    fn calculate(&mut self, kp: f32, ki: f32, kd: f32, error: f32, dt: f32, i_limit: f32) -> i16 {
        self.i += error * dt * ki;

        self.i = self.i.clamp(-i_limit, i_limit);

        // Do basic exponential low-pass on the D term
        let d_term = (error - self.last_error) * kd / dt;
        self.d_term_lpf = self.d_term_lpf * (1f32 - D_TERM_LPF_FACTOR) + d_term * D_TERM_LPF_FACTOR;

        let result = kp * error + self.i + self.d_term_lpf;
        self.last_error = error;

        result as i16
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

    last_pid_time: Instant,
}

impl PidContext {
    pub fn new() -> Self {
        PidContext {
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

pub async fn do_pid_iteration(
    armed: bool,
    context: &mut PidContext,
    inputs: &CRSFChannels,
    pid_inputs: PidInputs,
) -> MotorInputs {
    let now = Instant::now();
    let dt = (now - context.last_pid_time).as_micros() as f32 * 1e-6;
    context.last_pid_time = now;

    let motor_thrust = inputs.throttle();

    if !armed {
        MotorInputs::disarmed()
    } else if motor_thrust < 200 {
        context.reset();

        MotorInputs::idle(motor_thrust, inputs.wing_roll(), inputs.wing_pitch())
    } else {
        let command_limit = (motor_thrust / 4) as i16;

        let motion_data = pid_inputs.motion_data;
        let target_velocities = pid_inputs.target_angular_velocities;

        let yaw_error = target_velocities.yaw - motion_data.gyro_yaw;
        let pitch_error = target_velocities.pitch - motion_data.gyro_pitch;
        let roll_error = target_velocities.roll - motion_data.gyro_roll;

        let yaw_input = context
            .yaw_pid
            .calculate(YAW_KP, YAW_KI, YAW_KD, yaw_error, dt, command_limit as f32)
            .clamp(-command_limit, command_limit);

        let pitch_input = context
            .pitch_pid
            .calculate(
                PITCH_KP * inputs.master_pi(),
                PITCH_KI * inputs.master_pi(),
                PITCH_KD * inputs.master_d(),
                pitch_error,
                dt,
                command_limit as f32,
            )
            .clamp(-command_limit, command_limit);

        let roll_input = context
            .roll_pid
            .calculate(
                ROLL_KP * inputs.master_pi(),
                ROLL_KI * inputs.master_pi(),
                ROLL_KD * inputs.master_d(),
                roll_error,
                dt,
                command_limit as f32,
            )
            .clamp(-command_limit, command_limit);

        MotorInputs {
            motor_thrust,
            yaw_input,
            pitch_input,
            roll_input,
            servo_pitch: inputs.wing_pitch(),
            servo_roll: inputs.wing_roll(),
        }
    }
}
