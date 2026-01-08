use embassy_stm32::{
    gpio::{AnyPin, Level, Output, Pin},
    pac::gpio::Gpio,
    Peri,
};
use embassy_time::{Duration, Instant, Timer};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::{
    dshot::{dshot_send_parallel, dshot_send_single},
    hal::mcu_utils::get_pin_gpio,
    logging::info,
    mixer::MotorMix,
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
pub enum BeepTone {
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

#[allow(dead_code)]
#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug)]
#[repr(u8)]
pub enum MotorDirection {
    Forward,
    Backward,
}

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
}

pub struct MotorInputs {
    pub motor_thrust: u16,
    pub yaw_input: i16,
    pub roll_input: i16,
    pub pitch_input: i16,

    pub left_aileron: i16,
    pub right_aileron: i16,
    pub elevator: i16,
}

impl MotorInputs {
    pub fn idle(motor_thrust: u16) -> Self {
        MotorInputs {
            motor_thrust,
            yaw_input: 0,
            roll_input: 0,
            pitch_input: 0,
            right_aileron: 0,
            left_aileron: 0,
            elevator: 0,
        }
    }
}

pub struct Motor {
    port: Gpio,
    pin: u8,
    _output: Output<'static>,
}

impl Motor {
    pub fn new(pin: Peri<'static, AnyPin>) -> Self {
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

    pub fn beep(&self, tone: BeepTone) {
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

    async fn multi_set_setting<const COUNT: usize>(
        motors: [&Self; COUNT],
        settings: [DshotCommand; COUNT],
    ) {
        let settings_as_u16 = settings.map(|setting| setting as u16);

        for _ in 1..1000 {
            Motor::multi_send(motors, [DshotCommand::DSHOT_CMD_STOP as u16; COUNT]);
            Timer::after_millis(1).await;
        }

        for _ in 1..10 {
            Motor::multi_send(motors, settings_as_u16);
            Timer::after_millis(1).await;
        }

        for _ in 1..10 {
            Motor::multi_send(
                motors,
                [DshotCommand::DSHOT_CMD_SAVE_SETTINGS as u16; COUNT],
            );
            Timer::after_millis(1).await;
        }
        Timer::after_millis(12).await;
        for _ in 1..1000 {
            Motor::multi_send(motors, [DshotCommand::DSHOT_CMD_STOP as u16; COUNT]);
            Timer::after_millis(1).await;
        }
    }

    async fn multi_set_direction<const COUNT: usize>(
        motors: [&Self; COUNT],
        directions: [MotorDirection; COUNT],
    ) {
        let directions_as_commands = directions.map(|direction| match direction {
            MotorDirection::Forward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_1,
            MotorDirection::Backward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_2,
        });

        Self::multi_set_setting(motors, directions_as_commands).await
    }

    async fn multi_disable_3d_mode<const COUNT: usize>(motors: [&Self; COUNT]) {
        Self::multi_set_setting(motors, [DshotCommand::DSHOT_CMD_3D_MODE_OFF; COUNT]).await
    }

    pub fn multi_throttle<const COUNT: usize>(motors: [&Self; COUNT], mut throttles: [u16; COUNT]) {
        for throttle in &mut throttles {
            if *throttle > 0 {
                *throttle += 48;
            }
        }

        Self::multi_send(motors, throttles);
    }

    fn multi_send<const COUNT: usize>(motors: [&Self; COUNT], values: [u16; COUNT]) {
        let same_port = motors.iter().all(|motor| motor.port == motors[0].port);

        if same_port {
            dshot_send_parallel(
                motors[0].port.bsrr(),
                motors.map(|motor| motor.pin as usize),
                values,
            );
        } else {
            for (motor, value) in motors.iter().zip(values) {
                motor.send_value(value);
            }
        }
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

            Motor::multi_throttle(motors.each_ref(), throttles);
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
    let motors = [front_left, front_right, rear_left, rear_right];

    let directions = [
        config.front_left_direction,
        config.front_right_direction,
        config.rear_left_direction,
        config.rear_right_direction,
    ];
    for i in 0..5 {
        info!("Applying directions {i} of 5");

        Motor::multi_disable_3d_mode(motors).await;
        Motor::multi_set_direction(motors, directions).await;
    }
}
