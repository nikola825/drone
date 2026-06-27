use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, little_endian};

use crate::configurator_protocol::protocol::Paddable;

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct BatteryResponse {
    pub voltage: little_endian::F32,
    pub cell_count: u8,
}

impl Paddable for BatteryResponse {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes, Default, Clone, Copy)]
#[repr(u8)]
pub enum FcPhase {
    #[default]
    IMUInitFail,
    Disarmed,
    Armed,
    Config
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes, Default, Clone, Copy)]
#[repr(C)]
pub struct FcStatus {
    pub valid: bool,
    pub total_duration: little_endian::F32,
    pub inner_duration: little_endian::F32,
    pub min_measured_period: little_endian::U64,
    pub max_measured_period: little_endian::U64,
    pub fc_phase: FcPhase
}

impl Paddable for FcStatus {}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes, Clone, Copy)]
#[repr(C)]
pub struct MotorOverride {
    pub motor_index: u8,
    pub direction: MotorOverrideDirection
}

impl Paddable for MotorOverride {

}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes, Clone, Copy)]
#[repr(u8)]
pub enum MotorOverrideDirection {
    Forward,
    Backward
}
