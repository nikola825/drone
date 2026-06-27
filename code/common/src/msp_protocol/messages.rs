use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, little_endian};

use crate::msp_protocol::protocol::{MSPMessagePayload, MSPMessageType, MSPReceivedPayload};

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct FcVariantMessage<const STRING_LEN: usize> {
    variant: [u8; STRING_LEN],
}

impl<const STRING_LEN: usize> MSPMessagePayload for FcVariantMessage<STRING_LEN> {
    fn message_type() -> MSPMessageType {
        MSPMessageType::FC_VARIANT
    }
}

impl<const STRING_LEN: usize> FcVariantMessage<STRING_LEN> {
    pub fn new(variant: &[u8; STRING_LEN]) -> Self {
        Self { variant: *variant }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct FcStatusMessage {
    unused_data_1: [u8; 6],
    armed: u8,
    unused_data_2: [u8; 15],
}

impl MSPMessagePayload for FcStatusMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::STATUS
    }
}

impl FcStatusMessage {
    pub fn new(armed: bool) -> Self {
        let armed = if armed { 1u8 } else { 0u8 };

        FcStatusMessage {
            armed,
            unused_data_1: Default::default(),
            unused_data_2: Default::default(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct SticksMessage {
    pub roll: little_endian::U16,
    pub pitch: little_endian::U16,
    pub yaw: little_endian::U16,
    pub throttle: little_endian::U16,
}

impl MSPMessagePayload for SticksMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::RC
    }
}

impl Default for SticksMessage {
    fn default() -> Self {
        SticksMessage {
            roll: 1500.into(),
            pitch: 1500.into(),
            yaw: 1500.into(),
            throttle: 1500.into(),
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct ApiVersionMessage {
    pub protocol_version: u8,
    pub api_version_major: u8,
    pub api_version_minor: u8,
}

impl MSPMessagePayload for ApiVersionMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_API_VERSION
    }
}

impl MSPReceivedPayload for ApiVersionMessage {}

impl Default for ApiVersionMessage {
    fn default() -> Self {
        Self {
            protocol_version: 0,
            api_version_major: 1,
            api_version_minor: 48,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, Default)]
#[repr(C)]
pub struct BatteryStateMessage {
    cell_count: u8,
    battery_capacity_mah: little_endian::U16,
    battery_voltage_div_10: u8,
    mah_drawn: little_endian::U16,
    amperage_div_100: little_endian::U16,
    battery_alerts: u8,
    battery_voltage_div_100: little_endian::U16,
}

impl MSPMessagePayload for BatteryStateMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_BATTERY_STATE
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned, TryFromBytes)]
#[repr(C)]
pub struct MotorConfigMessage {
    min_throttle: little_endian::U16,
    max_throttle: little_endian::U16,
    min_command: little_endian::U16,
    pub motor_count: u8,
    pole_count: u8,
    use_dshot_telemetry: bool,
    esc_sensor_available: bool,
}

impl MSPMessagePayload for MotorConfigMessage {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_MOTOR_CONFIG
    }
}

impl MSPReceivedPayload for MotorConfigMessage {

}

impl MotorConfigMessage {
    pub fn new(motor_count: u8) -> Self {
        MotorConfigMessage {
            min_throttle: 0.into(),
            max_throttle: 2000.into(),
            min_command: 1000.into(),
            motor_count,
            pole_count: 14,
            use_dshot_telemetry: false,
            esc_sensor_available: false,
        }
    }
}

#[derive(IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct SetPassthroughResponse {
    motor_count: u8,
}

impl MSPMessagePayload for SetPassthroughResponse {
    fn message_type() -> MSPMessageType {
        MSPMessageType::MSP_SET_PASSTHROUGH
    }
}

impl SetPassthroughResponse {
    pub fn new(motor_count: u8) -> Self {
        Self { motor_count }
    }
}
