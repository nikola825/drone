use crate::{
    crc8::crc8_calculate,
    logging::error,
    motors::{self, MotorDirection},
};

#[cfg(feature = "flash_storage")]
use crate::hw_select::STORED_CONFIG_START;

use embassy_stm32::flash::{Blocking, Flash};
use embassy_time::Timer;
use log::info;
use zerocopy::{little_endian::F32, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

pub const STORED_CONFIG_STRUCT_SIZE: usize = 64;

#[derive(Debug)]
pub enum StoredConfigError {
    #[allow(dead_code)]
    FlashError(embassy_stm32::flash::Error),
    FailedToDeserialize,
    ChecksumFailed,
    #[allow(dead_code)]
    ValidationError(&'static str),
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
    pub yaw_offset: F32,
    pub pitch_offset: F32,
    pub roll_offset: F32,
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
    fn validate(self) -> Result<Self, StoredConfigError> {
        if !self.validate_checksum() {
            Err(StoredConfigError::ChecksumFailed)
        } else if self.front_left_motor == self.front_right_motor {
            Err(StoredConfigError::ValidationError(
                "Front left and front right motors are equal",
            ))
        } else if self.front_left_motor == self.rear_left_motor {
            Err(StoredConfigError::ValidationError(
                "Front left and rear left motors are equal",
            ))
        } else if self.front_left_motor == self.rear_right_motor {
            Err(StoredConfigError::ValidationError(
                "Front left and rear right motors are equal",
            ))
        } else if self.front_right_motor == self.rear_left_motor {
            Err(StoredConfigError::ValidationError(
                "Front right and rear left motors are equal",
            ))
        } else if self.front_right_motor == self.rear_right_motor {
            Err(StoredConfigError::ValidationError(
                "Front right and rear right motors are equal",
            ))
        } else if self.rear_left_motor == self.rear_right_motor {
            Err(StoredConfigError::ValidationError(
                "Rear left and rear right motors are equal",
            ))
        } else {
            Ok(self)
        }
    }

    fn update_checksum(&mut self) {
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

#[allow(dead_code, clippy::field_reassign_with_default)]
#[cfg(feature = "flash_storage")]
pub async fn reconfigure_and_store(flash: &mut Flash<'static, Blocking>) {
    let mut config = StoredConfig::default();
    //let mut config = read_stored_config(flash).await;

    config.front_left_motor = 2;
    config.front_right_motor = 3;
    config.rear_left_motor = 1;
    config.rear_right_motor = 0;
    config.front_left_direction = motors::MotorDirection::Forward;
    config.front_right_direction = motors::MotorDirection::Backward;
    config.rear_left_direction = motors::MotorDirection::Backward;
    config.rear_right_direction = motors::MotorDirection::Forward;
    config.yaw_offset = (-0.11099324f32).into();
    config.pitch_offset = (0.51241034).into();
    config.roll_offset = (-0.051307525).into();

    store_config(flash, config).await;
}

pub async fn read_stored_config(flash: &mut Flash<'static, Blocking>) -> StoredConfig {
    #[cfg(feature = "flash_storage")]
    {
        Timer::after_millis(300).await;

        let mut buffer = [0u8; STORED_CONFIG_STRUCT_SIZE];

        let stored_config_result = flash
            .blocking_read(STORED_CONFIG_START as u32, &mut buffer)
            .map_err(StoredConfigError::FlashError)
            .and_then(|_| {
                StoredConfig::try_read_from_bytes(&buffer)
                    .map_err(|_| StoredConfigError::FailedToDeserialize)
            })
            .and_then(StoredConfig::validate);

        match stored_config_result {
            Ok(stored_config) => stored_config,
            Err(err) => loop {
                error!("Failed to read stored configuration {:?}", err);
                Timer::after_millis(100).await;
            },
        }
    }

    #[cfg(not(feature = "flash_storage"))]
    {
        StoredConfig {
            front_left_motor: 2,
            front_right_motor: 3,
            rear_left_motor: 1,
            rear_right_motor: 0,
            front_left_direction: motors::MotorDirection::Forward,
            front_right_direction: motors::MotorDirection::Backward,
            rear_left_direction: motors::MotorDirection::Backward,
            rear_right_direction: motors::MotorDirection::Forward,
            yaw_offset: (-0.11099324f32).into(),
            pitch_offset: (0.51241034).into(),
            roll_offset: (-0.051307525).into(),
            checksum: 0,
            ..Default::default()
        }
    }
}

#[cfg(feature = "flash_storage")]
pub async fn store_config(flash: &mut Flash<'static, Blocking>, mut config: StoredConfig) {
    config.update_checksum();
    Timer::after_millis(300).await;

    let store_result = config.validate().and_then(|config| {
        let bytes = config.as_bytes();
        flash
            .blocking_write(STORED_CONFIG_START as u32, bytes)
            .map_err(StoredConfigError::FlashError)
    });

    if let Err(err) = store_result {
        error!("Failed store configuration {:?}", err);
        Timer::after_millis(100).await;
    }
}

#[allow(dead_code)]
#[cfg(feature = "flash_storage")]
pub async fn dump_config(flash: &mut Flash<'static, Blocking>) {
    let config = read_stored_config(flash).await;

    loop {
        info!("CFG DUMP={:?}", config);
        Timer::after_millis(100).await;
    }
}
