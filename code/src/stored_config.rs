use crate::{
    crc8::crc8_calculate,
    hal::{ConfigStorageError, ConfigStore},
    logging::error,
    motors::MotorDirection,
};

use embassy_time::Timer;
use log::info;
use zerocopy::{little_endian::F32, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

pub const STORED_CONFIG_STRUCT_SIZE: u32 = 64;

#[derive(Debug)]
pub enum StoredConfigError {
    #[allow(dead_code)]
    StorageError(ConfigStorageError),
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

#[allow(dead_code, clippy::field_reassign_with_default)]
pub async fn reconfigure_and_store(storage: &mut impl ConfigStore) {
    let mut config = StoredConfig::default();
    //let mut config = read_stored_config(flash).await;

    config.front_left_motor = 1;
    config.front_right_motor = 2;
    config.rear_left_motor = 0;
    config.rear_right_motor = 3;
    config.front_left_direction = MotorDirection::Forward;
    config.front_right_direction = MotorDirection::Backward;
    config.rear_left_direction = MotorDirection::Backward;
    config.rear_right_direction = MotorDirection::Forward;
    config.yaw_offset = (-0.11099324f32).into();
    config.pitch_offset = (0.51241034).into();
    config.roll_offset = (-0.051307525).into();

    store_config(storage, config).await;
}

pub async fn read_stored_config(storage: &mut impl ConfigStore) -> StoredConfig {
    Timer::after_millis(300).await;

    let stored_config_result = storage
        .read_stored_config()
        .map_err(StoredConfigError::StorageError)
        .and_then(StoredConfig::validate);

    match stored_config_result {
        Ok(stored_config) => stored_config,
        Err(err) => loop {
            error!("Failed to read stored configuration {:?}", err);
            Timer::after_millis(100).await;
        },
    }
}

pub async fn store_config(storage: &mut impl ConfigStore, mut config: StoredConfig) {
    config.update_checksum();
    Timer::after_millis(300).await;

    let store_result = config.validate().and_then(|config| {
        storage
            .store_config(&config)
            .map_err(StoredConfigError::StorageError)
    });

    if let Err(err) = store_result {
        error!("Failed store configuration {:?}", err);
        Timer::after_millis(100).await;
    }
}

#[allow(dead_code)]
pub async fn dump_config(storage: &mut impl ConfigStore) {
    let config = read_stored_config(storage).await;

    loop {
        info!("CFG DUMP={:?}", config);
        Timer::after_millis(100).await;
    }
}
