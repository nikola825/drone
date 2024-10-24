use core::cmp::min;

use embassy_stm32::{
    gpio::{Level, Output, Pin},
    pac::GPIO,
};
use embassy_time::Timer;

use crate::{dshot::dshot_send, DroneContext};

pub struct Motor {
    port: u8,
    pin: u8,
    _output: Output<'static>,
}

pub struct MotorsContext {
    front_left: Motor,
    front_right: Motor,
    rear_left: Motor,
    rear_right: Motor,
    running: bool,
}

impl MotorsContext {
    pub fn new(front_left: Motor, front_right: Motor, rear_left: Motor, rear_right: Motor) -> Self {
        MotorsContext {
            front_left,
            front_right,
            rear_left,
            rear_right,
            running: false,
        }
    }
}

impl Motor {
    pub fn new(pin: impl Pin + 'static) -> Self {
        let port = pin.port();
        let pin_number = pin.pin();
        let output = Output::new(pin, Level::Low, embassy_stm32::gpio::Speed::VeryHigh);
        Motor {
            port: port,
            pin: pin_number,
            _output: output,
        }
    }

    fn send_value(&self, value: u16) {
        dshot_send(GPIO(self.port as _).bsrr(), self.pin as _, value);
    }

    pub fn set_throttle(&self, throttle: u16) {
        if throttle > 0 {
            self.send_value(48 + throttle);
        } else {
            self.send_value(0);
        }
    }

    #[allow(dead_code)]
    pub async fn set_direction_0(&self) {
        for _ in 1..100 {
            self.send_value(0);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(7);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(12);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(0);
            Timer::after_millis(10).await;
        }
    }

    #[allow(dead_code)]
    pub async fn set_direction_1(&self) {
        for _ in 1..100 {
            self.send_value(0);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(8);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(12);
            Timer::after_millis(10).await;
        }
        Timer::after_millis(10).await;
        for _ in 1..100 {
            self.send_value(0);
            Timer::after_millis(10).await;
        }
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

    zero_throttle(&context);
    context.running = false;
}

fn zero_throttle(context: &MotorsContext) {
    context.front_left.set_throttle(0);
    context.front_right.set_throttle(0);
    context.rear_left.set_throttle(0);
    context.rear_right.set_throttle(0);
}

pub async fn disarm(context: &mut DroneContext) {
    if context.motor_context.running {
        gentle_stop(
            context.navigation_context.motor_thrust/4,
            &mut context.motor_context,
        )
        .await;
    } else {
        zero_throttle(&context.motor_context);
    }
}

pub fn drive(context: &mut DroneContext) {
    if context.armed {
        context.motor_context.running = true;

        let thrust = context.navigation_context.motor_thrust as i16;

        let yaw_input = context.navigation_context.yaw_input;
        let pitch_input = context.navigation_context.pitch_input;
        let roll_input = context.navigation_context.roll_input;

        let front_left: i16 = (thrust + roll_input - pitch_input + yaw_input) /4;
        let front_right: i16 = (thrust - roll_input - pitch_input - yaw_input) /4;
        let rear_left: i16 = (thrust + roll_input + pitch_input - yaw_input) /4;
        let rear_right: i16 = (thrust - roll_input + pitch_input + yaw_input) /4;

        context.motor_context.front_left.set_throttle(min(front_left as u16, 1990));
        context.motor_context.front_right.set_throttle(min(front_right as u16, 1990));
        context.motor_context.rear_left.set_throttle(min(rear_left as u16, 1990));
        context.motor_context.rear_right.set_throttle(min(rear_right as u16, 1990));
    } else {
        zero_throttle(&context.motor_context);
    }
}
