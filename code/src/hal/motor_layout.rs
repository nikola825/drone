use crate::{hal::servo::ServoDriver, motors::Motor};

#[allow(dead_code)]
pub struct QuadcopterLayout {
    pub motors: [Motor; 4],
}

#[allow(dead_code)]
pub struct WingLayout {
    pub thrust_motor: Motor,
    pub left_winglet_servo: ServoDriver,
    pub right_winglet_servo: ServoDriver,
}

#[cfg(feature = "quad")]
pub use QuadcopterLayout as MotorLayout;

#[cfg(feature = "wing")]
pub use WingLayout as MotorLayout;
