use embassy_stm32::{
    time::Hertz,
    timer::{
        simple_pwm::{PwmPin, SimplePwm, SimplePwmChannel},
        Ch1, Ch2, Ch3, Ch4,
    },
    Peri,
};

use crate::hal::SERVO_TIMER;

const SERVO_MAX_US: u32 = 2500;
const SERVO_MIN_US: u32 = 500;

#[allow(dead_code)]
pub struct ServoDriverMaker {
    frequency: Hertz,
    channels: [Option<SimplePwmChannel<'static, SERVO_TIMER>>; 4],
}

#[allow(dead_code)]
impl ServoDriverMaker {
    pub fn new(
        ch1: Option<PwmPin<'static, SERVO_TIMER, Ch1>>,
        ch2: Option<PwmPin<'static, SERVO_TIMER, Ch2>>,
        ch3: Option<PwmPin<'static, SERVO_TIMER, Ch3>>,
        ch4: Option<PwmPin<'static, SERVO_TIMER, Ch4>>,
        timer: Peri<'static, SERVO_TIMER>,
        frequency: Hertz,
    ) -> Self {
        let channel_set = [ch1.is_some(), ch2.is_some(), ch3.is_some(), ch4.is_some()];

        let pwm = SimplePwm::new(timer, ch1, ch2, ch3, ch4, frequency, Default::default());

        let split = pwm.split();

        let channels = [
            if channel_set[0] {
                Some(split.ch1)
            } else {
                None
            },
            if channel_set[1] {
                Some(split.ch2)
            } else {
                None
            },
            if channel_set[2] {
                Some(split.ch3)
            } else {
                None
            },
            if channel_set[3] {
                Some(split.ch4)
            } else {
                None
            },
        ];

        Self {
            frequency,
            channels,
        }
    }

    pub fn make_channel(&mut self, channel: embassy_stm32::timer::Channel) -> ServoDriver {
        let index = match channel {
            embassy_stm32::timer::Channel::Ch1 => 0,
            embassy_stm32::timer::Channel::Ch2 => 1,
            embassy_stm32::timer::Channel::Ch3 => 2,
            embassy_stm32::timer::Channel::Ch4 => 3,
        };

        let mut channel = self.channels[index].take().unwrap();

        channel.enable();

        ServoDriver::new(channel, self.frequency)
    }
}

pub struct ServoDriver {
    channel: SimplePwmChannel<'static, SERVO_TIMER>,

    duty_max: u16,
    duty_min: u16,
}

impl ServoDriver {
    pub fn new(channel: SimplePwmChannel<'static, SERVO_TIMER>, frequency: Hertz) -> Self {
        let period_us = 1000000u32 / frequency.0;

        let divider_min = period_us / SERVO_MIN_US;
        let divider_max = period_us / SERVO_MAX_US;

        let duty_max = ((channel.max_duty_cycle() as u32) / divider_max) as u16;
        let duty_min = ((channel.max_duty_cycle() as u32) / divider_min) as u16;

        ServoDriver {
            channel,
            duty_max,
            duty_min,
        }
    }

    pub fn update(&mut self, angle: i16) {
        let mid = ((self.duty_max + self.duty_min) / 2) as i32;
        let range = (self.duty_max as i32) - mid;
        let delta = range * (angle as i32) / 90;

        let duty_cycle = ((mid + delta) as u16).clamp(self.duty_min, self.duty_max);

        self.channel.set_duty_cycle(duty_cycle);
    }
}
