#![cfg(feature = "five-inch")]

pub const YAW_KP: f32 = 100.0f32;
pub const YAW_KI: f32 = 72.0f32;
pub const YAW_KD: f32 = 0.0f32;

pub const PITCH_KP: f32 = 12.6f32;
pub const PITCH_KI: f32 = 60.0f32;
pub const PITCH_KD: f32 = 0.045f32;

pub const ROLL_KP: f32 = PITCH_KP;
pub const ROLL_KI: f32 = PITCH_KI;
pub const ROLL_KD: f32 = PITCH_KD;
