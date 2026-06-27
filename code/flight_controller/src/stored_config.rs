use crate::{
    hal::{ConfigStorageError, ConfigStore},
    //logging::error,
};

use common::shared_objects::{MotorDirection, StoredConfig};
use embassy_time::Timer;

pub const STORED_CONFIG_STRUCT_SIZE: u32 = 64;

#[derive(Debug)]
pub enum StoredConfigError {
    #[allow(dead_code)]
    StorageError(ConfigStorageError),
    #[allow(dead_code)]
    ValidationError(common::shared_objects::StoredConfigValidationError),
}

#[allow(dead_code, clippy::field_reassign_with_default)]
pub async fn reconfigure_and_store(
    storage: &mut impl ConfigStore,
) -> Result<(), StoredConfigError> {
    // let mut config = StoredConfig::default();
    let mut config = read_stored_config(storage).await?;

    config.front_left_motor = 0;
    config.front_right_motor = 1;
    config.rear_left_motor = 3;
    config.rear_right_motor = 2;
    config.front_left_direction = MotorDirection::Forward;
    config.front_right_direction = MotorDirection::Backward;
    config.rear_left_direction = MotorDirection::Backward;
    config.rear_right_direction = MotorDirection::Forward;
    config.yaw_offset = (-0.11099324f32).into();
    config.pitch_offset = (0.51241034).into();
    config.roll_offset = (-0.051307525).into();

    store_config(storage, config).await;

    Ok(())
}

pub async fn read_stored_config(
    storage: &mut impl ConfigStore,
) -> Result<StoredConfig, StoredConfigError> {
    Timer::after_millis(300).await;

    storage
        .read_stored_config()
        .map_err(StoredConfigError::StorageError)
        .and_then(|config| {
            config
                .validate()
                .map_err(StoredConfigError::ValidationError)?;

            Ok(config)
        })
}

pub async fn store_config(storage: &mut impl ConfigStore, mut config: StoredConfig) {
    config.update_checksum();
    Timer::after_millis(300).await;

    let store_result = config
        .validate()
        .map_err(StoredConfigError::ValidationError)
        .and_then(|_| {
            storage
                .store_config(&config)
                .map_err(StoredConfigError::StorageError)
        });

    if let Err(_err) = store_result {
        loop {
            //error!("Failed to store configuration {:?}", err);
            Timer::after_millis(100).await;
        }
    }
}

#[allow(dead_code)]
pub async fn dump_config(storage: &mut impl ConfigStore) {
    let _config = read_stored_config(storage).await;

    loop {
        //info!("CFG DUMP={:?}", config);
        Timer::after_millis(100).await;
    }
}
