#[cfg(feature = "servo-support")]
use crate::hal::ServoDriver;
use crate::motors::Motor;

#[cfg(feature = "serial-passthrough")]
use crate::hal::EscSoftSerial;

#[allow(dead_code)]
pub struct QuadcopterLayout {
    pub motor0: Peri<'static, AnyPin>,
    pub motor1: Peri<'static, AnyPin>,
    pub motor2: Peri<'static, AnyPin>,
    pub motor3: Peri<'static, AnyPin>,
}

impl QuadcopterLayout {
    pub fn get_motors(self) -> [Motor; 4] {
        [
            Motor::new(self.motor0),
            Motor::new(self.motor1),
            Motor::new(self.motor2),
            Motor::new(self.motor3),
        ]
    }

    #[cfg(feature = "serial-passthrough")]
    pub fn get_serials(self) -> [EscSoftSerial; 4] {
        [
            EscSoftSerial::new(self.motor0),
            EscSoftSerial::new(self.motor1),
            EscSoftSerial::new(self.motor2),
            EscSoftSerial::new(self.motor3),
        ]
    }
}

#[cfg(feature = "servo-support")]
#[allow(dead_code)]
pub struct WingLayout {
    pub thrust_motor: Motor,
    pub left_winglet_servo: ServoDriver,
    pub right_winglet_servo: ServoDriver,
}

#[cfg(feature = "quad")]
pub use QuadcopterLayout as MotorLayout;

use embassy_stm32::{gpio::AnyPin, Peri};
#[cfg(feature = "wing")]
pub use WingLayout as MotorLayout;
