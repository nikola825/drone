use crate::esc::{EscMotor, EscMotorSet};
#[cfg(feature = "servo-support")]
use crate::hal::{ServoDriver, DSHOT_DMA, DSHOT_TIMER};

#[allow(dead_code)]
pub struct QuadcopterLayout {
    pub motor0: Peri<'static, AnyPin>,
    pub motor1: Peri<'static, AnyPin>,
    pub motor2: Peri<'static, AnyPin>,
    pub motor3: Peri<'static, AnyPin>,
    pub dshot_gpio: Gpio,
    pub dshot_dma: Peri<'static, DSHOT_DMA>,
    pub dshot_timer: Peri<'static, DSHOT_TIMER>,
}

impl QuadcopterLayout {
    pub fn get_motors(self) -> EscMotorSet {
        EscMotorSet::new(
            [self.motor0, self.motor1, self.motor2, self.motor3],
            self.dshot_dma,
            self.dshot_timer,
            self.dshot_gpio,
        )
    }
}

#[cfg(feature = "servo-support")]
#[allow(dead_code)]
pub struct WingLayout {
    pub thrust_motor: EscMotor,
    pub left_winglet_servo: ServoDriver,
    pub right_winglet_servo: ServoDriver,
}

#[cfg(feature = "quad")]
pub use QuadcopterLayout as MotorLayout;

#[cfg(feature = "quad")]
pub const ESC_COUNT: usize = 4;

use embassy_stm32::{gpio::AnyPin, pac::gpio::Gpio, Peri};
#[cfg(feature = "wing")]
pub use WingLayout as MotorLayout;

#[cfg(feature = "wing")]
pub const ESC_COUNT: usize = 1;
