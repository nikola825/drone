use core::f32;

use nalgebra::UnitComplex;

pub const RAD_TO_DEG_FACTOR: f32 = 180f32 / f32::consts::PI;
pub const DEG_TO_RAD_FACTOR: f32 = f32::consts::PI / 180f32;

pub fn angle_add(a1_deg: f32, a2_deg: f32) -> f32 {
    UnitComplex::new((a1_deg + a2_deg) * DEG_TO_RAD_FACTOR).angle() * RAD_TO_DEG_FACTOR
}

pub fn angle_sub(a1_deg: f32, a2_deg: f32) -> f32 {
    UnitComplex::new(a2_deg * DEG_TO_RAD_FACTOR)
        .angle_to(&UnitComplex::new(a1_deg * DEG_TO_RAD_FACTOR))
        * RAD_TO_DEG_FACTOR
}
