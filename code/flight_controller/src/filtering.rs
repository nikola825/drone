use core::f32;

use crate::icm42688::MotionData;
use num_traits::Float;

// Basic first order RC filter
// f_cutoff = 1 / (2*pi*rc)
// rc = 1/(2*pi*f_cutoff)
// smoothing_factor = sampling_time/(rc + sampling_time)
// delay = rc
#[allow(dead_code)]
pub struct RCFilter {
    state: f32,
    smoothing_factor: f32,
}

#[allow(dead_code)]
impl RCFilter {
    pub const fn new(cutoff_frequency: f32, sampling_period: f32) -> Self {
        let rc = 1f32 / (2f32 * f32::consts::PI * cutoff_frequency);
        let smoothing_factor = sampling_period / (rc + sampling_period);

        Self {
            state: 0f32,
            smoothing_factor,
        }
    }

    pub fn apply(&mut self, input: f32) -> f32 {
        self.state += self.smoothing_factor * (input - self.state);

        self.state
    }

    pub fn reset(&mut self) {
        self.state = 0f32;
    }
}

// SVF filter
// Dark magic, copied from Betaflight
// Works better than RC at the cost of slightly higher delay
pub struct SVFilter {
    f: f32,
    a1: f32,
    a2: f32,
    ic1: f32,
    ic2: f32,
}

impl SVFilter {
    pub fn new(cutoff: f32, sampling_period: f32) -> Self {
        const INV_BUTTERWORTH_Q: f32 = f32::consts::SQRT_2;
        let sn = (f32::consts::PI * cutoff * sampling_period).sin();
        let cs = (f32::consts::PI * cutoff * sampling_period).cos();

        let f = sn / cs;
        let inv_denom = 1.0 / (1.0 + f * (f + INV_BUTTERWORTH_Q));
        let a1 = inv_denom;
        let a2 = f * inv_denom;

        SVFilter {
            f,
            a1,
            a2,
            ic1: 0f32,
            ic2: 0f32,
        }
    }

    pub fn apply(&mut self, input: f32) -> f32 {
        let v3 = input - self.ic2;
        let v1 = self.a1 * self.ic1 + self.a2 * v3;
        let v2 = self.ic2 + self.f * v1;

        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;

        v2
    }

    pub fn reset(&mut self) {
        self.ic1 = 0f32;
        self.ic2 = 0f32;
    }
}

pub struct GyroFilter {
    yaw: SVFilter,
    pitch: SVFilter,
    roll: SVFilter,
}

impl GyroFilter {
    pub fn new(cutoff_frequency: f32, sampling_period: f32) -> Self {
        GyroFilter {
            yaw: SVFilter::new(cutoff_frequency, sampling_period),
            pitch: SVFilter::new(cutoff_frequency, sampling_period),
            roll: SVFilter::new(cutoff_frequency, sampling_period),
        }
    }

    pub fn apply(&mut self, mut data: MotionData) -> MotionData {
        data.gyro_yaw = self.yaw.apply(data.gyro_yaw);
        data.gyro_pitch = self.pitch.apply(data.gyro_pitch);
        data.gyro_roll = self.roll.apply(data.gyro_roll);

        data
    }

    pub fn reset(&mut self) {
        self.yaw.reset();
        self.pitch.reset();
        self.roll.reset();
    }
}
