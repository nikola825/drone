use embassy_stm32::{
    gpio::{Level, Output, Pin},
    pac::gpio::Gpio,
};
use embassy_time::{Duration, Instant, Timer};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    dshot::{dshot_send_parallel, dshot_send_single},
    hal::mcu_utils::get_pin_gpio,
    logging::info,
    stored_config::StoredConfig,
};

#[derive(Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
enum DshotCommand {
    DSHOT_CMD_STOP = 0,
    DSHOT_CMD_BEEP1 = 1,
    DSHOT_CMD_BEEP2 = 2,
    DSHOT_CMD_BEEP3 = 3,
    DSHOT_CMD_BEEP4 = 4,
    DSHOT_CMD_BEEP5 = 5,
    DSHOT_CMD_SPIN_DIRECTION_1 = 7,
    DSHOT_CMD_SPIN_DIRECTION_2 = 8,
    DSHOT_CMD_3D_MODE_OFF = 9,
    DSHOT_CMD_SAVE_SETTINGS = 12,
}

#[derive(Clone, Copy)]
enum BeepTone {
    Tone1,
    Tone2,
    Tone3,
    Tone4,
    Tone5,
}

impl BeepTone {
    fn next(self) -> Self {
        use BeepTone::*;
        match self {
            Tone1 => Tone2,
            Tone2 => Tone3,
            Tone3 => Tone4,
            Tone4 => Tone5,
            Tone5 => Tone1,
        }
    }
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug)]
#[repr(u8)]
pub enum MotorDirection {
    Forward,
    Backward,
}

pub struct MotorsContext {
    front_left: Motor,
    front_right: Motor,
    rear_left: Motor,
    rear_right: Motor,
    running: bool,
    beep_interval_start: Instant,
    beep_tone: BeepTone,
}

impl MotorsContext {
    pub fn new(front_left: Motor, front_right: Motor, rear_left: Motor, rear_right: Motor) -> Self {
        MotorsContext {
            front_left,
            front_right,
            rear_left,
            rear_right,
            running: false,
            beep_interval_start: Instant::MIN,
            beep_tone: BeepTone::Tone1,
        }
    }
}

pub struct MotorInputs {
    pub motor_thrust: u16,
    pub yaw_input: i16,
    pub roll_input: i16,
    pub pitch_input: i16,
}

impl MotorInputs {
    pub fn idle(motor_thrust: u16) -> Self {
        MotorInputs {
            motor_thrust,
            yaw_input: 0,
            roll_input: 0,
            pitch_input: 0,
        }
    }
}

pub struct Motor {
    port: Gpio,
    pin: u8,
    _output: Output<'static>,
}

impl Motor {
    pub fn new(pin: impl Pin + 'static) -> Self {
        let port = get_pin_gpio(&pin);
        let pin_number = pin.pin();
        let output = Output::new(pin, Level::Low, embassy_stm32::gpio::Speed::VeryHigh);
        Motor {
            port,
            pin: pin_number,
            _output: output,
        }
    }

    fn send_value(&self, value: u16) {
        dshot_send_single(self.port.bsrr(), self.pin as _, value);
    }

    fn send_command(&self, command: DshotCommand) {
        self.send_value(command as u16);
    }

    #[allow(dead_code)]
    fn set_throttle(&self, throttle: u16) {
        if throttle > 0 {
            self.send_value(48 + throttle);
        } else {
            self.send_value(0);
        }
    }

    fn beep(&self, tone: BeepTone) {
        use BeepTone::*;
        use DshotCommand::*;

        let command = match tone {
            Tone1 => DSHOT_CMD_BEEP1,
            Tone2 => DSHOT_CMD_BEEP2,
            Tone3 => DSHOT_CMD_BEEP3,
            Tone4 => DSHOT_CMD_BEEP4,
            Tone5 => DSHOT_CMD_BEEP5,
        };

        self.send_command(command);
    }

    #[allow(dead_code)]
    async fn set_setting_and_save(&self, setting: DshotCommand) {
        for _ in 1..100 {
            self.send_command(DshotCommand::DSHOT_CMD_STOP);
            Timer::after_millis(2).await;
        }

        Timer::after_millis(2).await;
        for _ in 1..100 {
            self.send_command(setting);
            Timer::after_millis(2).await;
        }

        Timer::after_millis(2).await;
        for _ in 1..100 {
            self.send_command(DshotCommand::DSHOT_CMD_SAVE_SETTINGS);
            Timer::after_millis(2).await;
        }

        Timer::after_millis(2).await;
        for _ in 1..100 {
            self.send_command(DshotCommand::DSHOT_CMD_STOP);
            Timer::after_millis(2).await;
        }
    }

    #[allow(dead_code)]
    async fn set_direction(&self, direction: MotorDirection) {
        let direction_command = match direction {
            MotorDirection::Forward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_1,
            MotorDirection::Backward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_2,
        };

        self.set_setting_and_save(direction_command).await;
    }

    #[allow(dead_code)]
    async fn disable_3d_mode(&self) {
        self.set_setting_and_save(DshotCommand::DSHOT_CMD_3D_MODE_OFF)
            .await;
    }

    fn multi_throttle(motors: [&Self; 4], mut throttles: [u16; 4]) {
        for throttle in &mut throttles {
            if *throttle > 0 {
                *throttle += 48;
            }
        }

        Self::multi_send(motors, throttles);
    }

    fn multi_send(motors: [&Self; 4], values: [u16; 4]) {
        if (motors[0].port == motors[1].port)
            && (motors[0].port == motors[2].port)
            && (motors[0].port == motors[3].port)
        {
            // If all the motors are on the same port
            // We can do a parallel bitbang
            dshot_send_parallel(
                motors[0].port.bsrr(),
                [
                    motors[0].pin as _,
                    motors[1].pin as _,
                    motors[2].pin as _,
                    motors[3].pin as _,
                ],
                values,
            );
        } else {
            // If the motors are on different ports, bitbang them independently
            motors[0].send_value(values[0]);
            motors[1].send_value(values[1]);
            motors[2].send_value(values[2]);
            motors[3].send_value(values[3]);
        }
    }
}

async fn gentle_stop(current_thrust: u16, context: &mut MotorsContext) {
    let mut thrust_target = current_thrust;

    while thrust_target > 200 {
        Motor::multi_throttle(
            [
                &context.front_left,
                &context.front_right,
                &context.rear_left,
                &context.rear_right,
            ],
            [thrust_target; 4],
        );

        Timer::after_millis(100).await;

        thrust_target = thrust_target * 70 / 100;
    }

    zero_throttle(context);
    context.running = false;
}

fn zero_throttle(context: &MotorsContext) {
    Motor::multi_throttle(
        [
            &context.front_left,
            &context.front_right,
            &context.rear_left,
            &context.rear_right,
        ],
        [0; 4],
    );
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
        context.front_left.beep(context.beep_tone);
        context.front_right.beep(context.beep_tone);
        context.rear_left.beep(context.beep_tone);
        context.rear_right.beep(context.beep_tone);
    }
}

pub async fn disarm(context: &mut MotorsContext, inputs: &MotorInputs, beep: bool) {
    if context.running {
        gentle_stop(inputs.motor_thrust / 4, context).await;
    } else if beep {
        beep_motors(context);
    } else {
        zero_throttle(context);
    }
}

pub fn drive_motors(context: &mut MotorsContext, inputs: &MotorInputs) {
    context.running = true;
    if inputs.motor_thrust > 0 {
        let thrust = inputs.motor_thrust as i16;

        let yaw_input = inputs.yaw_input;
        let pitch_input = inputs.pitch_input;
        let roll_input = inputs.roll_input;

        let front_left: i16 = (thrust + roll_input - pitch_input - yaw_input) / 4;
        let front_right: i16 = (thrust - roll_input - pitch_input + yaw_input) / 4;
        let rear_left: i16 = (thrust + roll_input + pitch_input + yaw_input) / 4;
        let rear_right: i16 = (thrust - roll_input + pitch_input - yaw_input) / 4;

        Motor::multi_throttle(
            [
                &context.front_left,
                &context.front_right,
                &context.rear_left,
                &context.rear_right,
            ],
            [
                front_left.clamp(0, 1990) as u16,
                front_right.clamp(0, 1990) as u16,
                rear_left.clamp(0, 1990) as u16,
                rear_right.clamp(0, 1990) as u16,
            ],
        );
    } else {
        zero_throttle(context);
    }
}

#[allow(dead_code)]
pub async fn do_motor_mapping(mut motors: [Option<Motor>; 4], config: &StoredConfig) -> ! {
    let motors = [
        motors[0].take().unwrap(),
        motors[1].take().unwrap(),
        motors[2].take().unwrap(),
        motors[3].take().unwrap(),
    ];

    let mut selected_motor = 0;
    loop {
        let motor_name = if selected_motor == config.front_left_motor {
            "front_left"
        } else if selected_motor == config.front_right_motor {
            "front_right"
        } else if selected_motor == config.rear_left_motor {
            "rear_left"
        } else if selected_motor == config.rear_right_motor {
            "rear_right"
        } else {
            "unknown"
        };

        info!("Doing motor {}: {}", selected_motor, motor_name);

        for _ in 0..2000 {
            let mut throttles = [0; 4];
            throttles[selected_motor as usize] = 60;

            Motor::multi_throttle([&motors[0], &motors[1], &motors[2], &motors[3]], throttles);
            Timer::after_micros(1005).await;
        }
        selected_motor = (selected_motor + 1) % 4;
    }
}

#[allow(dead_code)]
pub async fn apply_motor_directions(
    front_left: &Motor,
    front_right: &Motor,
    rear_left: &Motor,
    rear_right: &Motor,
    config: &StoredConfig,
) {
    for _ in 0..5 {
        info!("Applying directions");
        info!("Front left");
        front_left.disable_3d_mode().await;
        front_left.set_direction(config.front_left_direction).await;

        info!("Front right");
        front_right.disable_3d_mode().await;
        front_right
            .set_direction(config.front_right_direction)
            .await;

        info!("Rear left");
        rear_left.disable_3d_mode().await;
        rear_left.set_direction(config.rear_left_direction).await;

        info!("Rear right");
        rear_right.disable_3d_mode().await;
        rear_right.set_direction(config.rear_right_direction).await;

        info!("Apply end");
        Timer::after_millis(100).await;
    }
}
