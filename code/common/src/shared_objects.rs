use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned, little_endian};

use crate::crc8::crc8_calculate;

#[derive(Debug)]
pub enum StoredConfigValidationError {
    ChecksumFailed,
    ValidationError(&'static str),
}

#[allow(dead_code)]
#[derive(
    IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug, PartialEq, Eq,
)]
#[repr(u8)]
pub enum MotorDirection {
    Forward,
    Backward,
}

impl core::fmt::Display for MotorDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string_value = match self {
            MotorDirection::Forward => "Forward",
            MotorDirection::Backward => "Backward",
        };

        write!(f, "{string_value}")
    }
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct StoredConfig {
    pub checksum: u8,
    pub motor_positions: MotorPositions,
    pub motor_directions: [MotorDirection; 4],
    pub yaw_offset: little_endian::F32,
    pub pitch_offset: little_endian::F32,
    pub roll_offset: little_endian::F32,
    pub _unused: [u8; 43],
}

#[derive(IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct MotorPositions {
    pub positions: [u8; 4],
}

impl Default for MotorPositions {
    fn default() -> Self {
        Self {
            positions: [0, 1, 2, 3],
        }
    }
}

impl core::ops::IndexMut<MotorPosition> for MotorPositions {
    fn index_mut(&mut self, position: MotorPosition) -> &mut Self::Output {
        &mut self.positions[position as usize]
    }
}

impl core::ops::Index<MotorPosition> for MotorPositions {
    type Output = u8;

    fn index(&self, position: MotorPosition) -> &Self::Output {
        &self.positions[position as usize]
    }
}

impl MotorPositions {
    pub fn get_position_of(&self, motor: u8) -> Option<MotorPosition> {
        for position in MotorPosition::all_values() {
            if self[position] == motor {
                return Some(position);
            }
        }

        return None;
    }
}

impl Default for StoredConfig {
    fn default() -> Self {
        Self {
            checksum: Default::default(),
            motor_positions: Default::default(),
            motor_directions: [MotorDirection::Forward; 4],
            yaw_offset: 0f32.into(),
            pitch_offset: 0f32.into(),
            roll_offset: 0f32.into(),
            _unused: [0u8; 43],
        }
    }
}

impl StoredConfig {
    pub fn validate_values(&self) -> Result<(), StoredConfigValidationError> {
        for index1 in MotorPosition::all_values() {
            for index2 in MotorPosition::all_values() {
                if index1 != index2 && self.motor_positions[index1] == self.motor_positions[index2]
                {
                    return Err(StoredConfigValidationError::ValidationError(
                        "Found two equal motor positions",
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), StoredConfigValidationError> {
        if !self.valid_checksum() {
            return Err(StoredConfigValidationError::ChecksumFailed);
        }

        self.validate_values()?;

        Ok(())
    }

    pub fn update_checksum(&mut self) {
        let bytes = self.as_bytes();
        let checksum = crc8_calculate(&bytes[1..]);

        self.checksum = checksum;
    }

    pub fn valid_checksum(&self) -> bool {
        let bytes = self.as_bytes();
        let checksum = crc8_calculate(&bytes[1..]);

        self.checksum == checksum
    }

    pub fn get_direction_of(&self, motor: u8) -> MotorDirection {
        self.motor_directions[motor as usize]
    }

    pub fn set_direction_of(&mut self, motor: u8, direction: MotorDirection) {
        self.motor_directions[motor as usize] = direction;
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

#[allow(dead_code)]
#[derive(
    IntoBytes, Immutable, TryFromBytes, KnownLayout, Unaligned, Clone, Copy, Debug, PartialEq,
)]
#[repr(u8)]
pub enum MotorPosition {
    FrontLeft = 0,
    FrontRight = 1,
    RearLeft = 2,
    RearRight = 3,
}

impl MotorPosition {
    pub fn all_values() -> [Self; 4] {
        [
            MotorPosition::FrontLeft,
            MotorPosition::FrontRight,
            MotorPosition::RearLeft,
            MotorPosition::RearRight,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            MotorPosition::FrontLeft => "Front left",
            MotorPosition::FrontRight => "Front right",
            MotorPosition::RearLeft => "Rear left",
            MotorPosition::RearRight => "Rear right",
        }
    }
}

impl core::fmt::Display for MotorPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string_value = self.as_str();

        write!(f, "{string_value}")
    }
}
