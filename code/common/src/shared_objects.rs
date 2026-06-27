use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, little_endian};

use crate::crc8::crc8_calculate;

#[derive(Debug)]
pub enum StoredConfigValidationError {
    ChecksumFailed,
    ValidationError(&'static str),
}

#[allow(dead_code)]
#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug)]
#[repr(u8)]
pub enum MotorDirection {
    Forward,
    Backward,
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Debug)]
#[repr(C)]
pub struct StoredConfig {
    pub checksum: u8,
    pub front_left_motor: u8,
    pub front_right_motor: u8,
    pub rear_left_motor: u8,
    pub rear_right_motor: u8,
    pub front_left_direction: MotorDirection,
    pub front_right_direction: MotorDirection,
    pub rear_left_direction: MotorDirection,
    pub rear_right_direction: MotorDirection,
    pub yaw_offset: little_endian::F32,
    pub pitch_offset: little_endian::F32,
    pub roll_offset: little_endian::F32,
    pub _unused: [u8; 43],
}


impl Default for StoredConfig {
    fn default() -> Self {
        Self {
            checksum: Default::default(),
            front_left_motor: 0,
            front_right_motor: 1,
            rear_left_motor: 2,
            rear_right_motor: 3,
            front_left_direction: MotorDirection::Forward,
            front_right_direction: MotorDirection::Forward,
            rear_left_direction: MotorDirection::Forward,
            rear_right_direction: MotorDirection::Forward,
            yaw_offset: 0f32.into(),
            pitch_offset: 0f32.into(),
            roll_offset: 0f32.into(),
            _unused: [0u8; 43],
        }
    }
}

impl StoredConfig {
    pub fn validate(&self) -> Result<(), StoredConfigValidationError> {
        if !self.validate_checksum() {
            Err(StoredConfigValidationError::ChecksumFailed)
        } else if self.front_left_motor == self.front_right_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Front left and front right motors are equal",
            ))
        } else if self.front_left_motor == self.rear_left_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Front left and rear left motors are equal",
            ))
        } else if self.front_left_motor == self.rear_right_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Front left and rear right motors are equal",
            ))
        } else if self.front_right_motor == self.rear_left_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Front right and rear left motors are equal",
            ))
        } else if self.front_right_motor == self.rear_right_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Front right and rear right motors are equal",
            ))
        } else if self.rear_left_motor == self.rear_right_motor {
            Err(StoredConfigValidationError::ValidationError(
                "Rear left and rear right motors are equal",
            ))
        } else {
            Ok(())
        }
    }

    pub fn update_checksum(&mut self) {
        let bytes = self.as_bytes();
        let checksum = crc8_calculate(&bytes[1..]);

        self.checksum = checksum;
    }

    fn validate_checksum(&self) -> bool {
        let bytes = self.as_bytes();
        let checksum = crc8_calculate(&bytes[1..]);

        self.checksum == checksum
    }
}

#[allow(dead_code)]
#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug)]
#[repr(u8)]
pub enum InitStatus {
    ImuInitFail,
    ConfigReadFail,
    FlightControlInProgress,
}
