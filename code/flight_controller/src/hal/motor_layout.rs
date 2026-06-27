#[cfg(feature = "servo-support")]
use crate::hal::ServoDriver;
use crate::motor::Motor;

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

#[cfg(feature = "quad")]
pub const ESC_COUNT: usize = 4;

use embassy_stm32::{gpio::AnyPin, Peri};
#[cfg(feature = "wing")]
pub use WingLayout as MotorLayout;

#[cfg(feature = "wing")]
pub const ESC_COUNT: usize = 4;
