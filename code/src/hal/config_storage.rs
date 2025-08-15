use embassy_stm32::flash::{Blocking, Flash};
use zerocopy::{IntoBytes, TryFromBytes};

use crate::stored_config::{StoredConfig, STORED_CONFIG_STRUCT_SIZE};

#[derive(Debug)]
pub enum ConfigStorageError {
    #[allow(dead_code)]
    FlashError(embassy_stm32::flash::Error),
    DeserializationError,
}

pub trait ConfigStore {
    fn store_config(&mut self, config: &StoredConfig) -> Result<(), ConfigStorageError>;
    fn read_stored_config(&mut self) -> Result<StoredConfig, ConfigStorageError>;
}

pub struct FlashConfigStore {
    flash: Flash<'static, Blocking>,
    erase_start_address: u32,
    flash_size: u32,
    config_start_address: u32,
}

impl FlashConfigStore {
    pub fn new(
        flash: Flash<'static, Blocking>,
        erase_start_address: u32,
        flash_size: u32,
        config_start_address: u32,
    ) -> Self {
        Self {
            flash,
            erase_start_address,
            flash_size,
            config_start_address,
        }
    }
}

impl ConfigStore for FlashConfigStore {
    fn store_config(&mut self, config: &StoredConfig) -> Result<(), ConfigStorageError> {
        self.flash
            .blocking_erase(self.erase_start_address, self.flash_size)
            .map_err(ConfigStorageError::FlashError)?;
        self.flash
            .blocking_write(self.config_start_address, config.as_bytes())
            .map_err(ConfigStorageError::FlashError)?;

        Ok(())
    }

    fn read_stored_config(&mut self) -> Result<StoredConfig, ConfigStorageError> {
        let mut buffer = [0u8; STORED_CONFIG_STRUCT_SIZE as usize];

        self.flash
            .blocking_read(self.config_start_address, &mut buffer)
            .map_err(ConfigStorageError::FlashError)
            .and_then(|_| {
                StoredConfig::try_read_from_bytes(&buffer)
                    .map_err(|_| ConfigStorageError::DeserializationError)
            })
    }
}

pub struct HardcodedConfigStore {
    pub config: StoredConfig,
}

impl ConfigStore for HardcodedConfigStore {
    fn store_config(&mut self, config: &StoredConfig) -> Result<(), ConfigStorageError> {
        // do nothing, we have no backking store
        self.config = config.clone();

        Ok(())
    }

    fn read_stored_config(&mut self) -> Result<StoredConfig, ConfigStorageError> {
        Ok(self.config.clone())
    }
}
