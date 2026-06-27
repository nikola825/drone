use core::f32;

use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;

use crate::{
    icm42688::{MotionData, IMU_ORIENTATION_MULTIPLIER},
    math_stuff::{DEG_TO_RAD_FACTOR, RAD_TO_DEG_FACTOR},
};

pub struct AhrsWrapper {
    inner: Madgwick<f32>,
}

pub struct AhrsAngles {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}

impl AhrsWrapper {
    pub fn new() -> Self {
        let madgwick = Madgwick::new(1f32 / 1000f32, 0.03);

        Self { inner: madgwick }
    }

    pub fn update(&mut self, motion_data: &MotionData) {
        let gyro_vec = Vector3::new(
            motion_data.gyro_pitch,
            motion_data.gyro_roll,
            motion_data.gyro_yaw,
        )
        .component_div(&IMU_ORIENTATION_MULTIPLIER)
            * DEG_TO_RAD_FACTOR;

        let accel_vec = Vector3::new(
            motion_data.accel_x,
            motion_data.accel_y,
            motion_data.accel_z,
        ) * 9.81;

        let _ = self.inner.update_imu(&gyro_vec, &accel_vec).unwrap();
    }

    pub fn read_ypr(&self) -> AhrsAngles {
        let euler = self.inner.quat.euler_angles();

        AhrsAngles {
            pitch: euler.0 * RAD_TO_DEG_FACTOR * IMU_ORIENTATION_MULTIPLIER.x,
            roll: euler.1 * RAD_TO_DEG_FACTOR * IMU_ORIENTATION_MULTIPLIER.y,
            yaw: euler.2 * RAD_TO_DEG_FACTOR * IMU_ORIENTATION_MULTIPLIER.z,
        }
    }
}
