use crate::{
    hal::{ConfigStorageError, ConfigStore},
    //logging::error,
};

use common::shared_objects::StoredConfig;
use embassy_time::Timer;

pub const STORED_CONFIG_STRUCT_SIZE: u32 = 64;

#[derive(Debug)]
pub enum StoredConfigError {
    #[allow(dead_code)]
    StorageError(ConfigStorageError),
    #[allow(dead_code)]
    ValidationError(common::shared_objects::StoredConfigValidationError),
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
