use core::cmp::min;

use embassy_stm32::{
    gpio::{Level, Output, Pin},
    pac::GPIO,
};
use embassy_time::{Duration, Instant, Timer};

use crate::dshot::{dshot_send, dshot_send_values};

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

#[allow(dead_code)]
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
    port: u8,
    pin: u8,
    _output: Output<'static>,
}

impl Motor {
    pub fn new(pin: impl Pin + 'static) -> Self {
        let port = pin.port();
        let pin_number = pin.pin();
        let output = Output::new(pin, Level::Low, embassy_stm32::gpio::Speed::VeryHigh);
        Motor {
            port,
            pin: pin_number,
            _output: output,
        }
    }

    fn send_value(&self, value: u16) {
        dshot_send(GPIO(self.port as _).bsrr(), self.pin as _, value);
    }

    fn send_command(&self, command: DshotCommand) {
        self.send_value(command as u16);
    }

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
    pub async fn set_direction(&self, direction: MotorDirection) {
        let direction_command = match direction {
            MotorDirection::Forward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_1,
            MotorDirection::Backward => DshotCommand::DSHOT_CMD_SPIN_DIRECTION_2,
        };

        self.set_setting_and_save(direction_command).await;
    }

    #[allow(dead_code)]
    pub async fn disable_3d_mode(&self) {
        self.set_setting_and_save(DshotCommand::DSHOT_CMD_3D_MODE_OFF)
            .await;
    }

    pub fn multi_throttle(
        motor0: &Self,
        motor1: &Self,
        motor2: &Self,
        motor3: &Self,
        mut throttle0: u16,
        mut throttle1: u16,
        mut throttle2: u16,
        mut throttle3: u16,
    ) {
        if throttle0 > 0 {
            throttle0 += 48;
        }

        if throttle1 > 0 {
            throttle1 += 48;
        }

        if throttle2 > 0 {
            throttle2 += 48;
        }

        if throttle3 > 0 {
            throttle3 += 48;
        }

        Self::multi_send(
            motor0, motor1, motor2, motor3, throttle0, throttle1, throttle2, throttle3,
        );
    }

    pub fn multi_send(
        motor0: &Self,
        motor1: &Self,
        motor2: &Self,
        motor3: &Self,
        value0: u16,
        value1: u16,
        value2: u16,
        value3: u16,
    ) {
        dshot_send_values(
            [
                GPIO(motor0.port as _).bsrr(),
                GPIO(motor1.port as _).bsrr(),
                GPIO(motor2.port as _).bsrr(),
                GPIO(motor3.port as _).bsrr(),
            ],
            [
                motor0.pin as _,
                motor1.pin as _,
                motor2.pin as _,
                motor3.pin as _,
            ],
            [value0, value1, value2, value3],
        );
    }
}

async fn gentle_stop(current_thrust: u16, context: &mut MotorsContext) {
    let mut thrust_target = current_thrust;

    while thrust_target > 200 {
        context.front_left.set_throttle(thrust_target);
        context.front_right.set_throttle(thrust_target);
        context.rear_left.set_throttle(thrust_target);
        context.rear_right.set_throttle(thrust_target);

        Timer::after_millis(100).await;

        thrust_target = thrust_target * 70 / 100;
    }

    zero_throttle(context);
    context.running = false;
}

fn zero_throttle(context: &MotorsContext) {
    Motor::multi_throttle(
        &context.front_left,
        &context.front_right,
        &context.rear_left,
        &context.rear_right,
        0,
        0,
        0,
        0,
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

pub fn drive(context: &mut MotorsContext, inputs: &MotorInputs) {
    context.running = true;
    if inputs.motor_thrust > 0 {
        let thrust = inputs.motor_thrust as i16;

        let yaw_input = inputs.yaw_input;
        let pitch_input = inputs.pitch_input;
        let roll_input = inputs.roll_input;

        let front_left: i16 = (thrust + roll_input - pitch_input + yaw_input) / 4;
        let front_right: i16 = (thrust - roll_input - pitch_input - yaw_input) / 4;
        let rear_left: i16 = (thrust + roll_input + pitch_input - yaw_input) / 4;
        let rear_right: i16 = (thrust - roll_input + pitch_input + yaw_input) / 4;

        Motor::multi_throttle(
            &context.front_left,
            &context.front_right,
            &context.rear_left,
            &context.rear_right,
            min(front_left as u16, 1990),
            min(front_right as u16, 1990),
            min(rear_left as u16, 1990),
            min(rear_right as u16, 1990),
        );
    } else {
        zero_throttle(context);
    }
}
