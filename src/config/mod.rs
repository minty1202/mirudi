mod data;
pub use data::ConfigScopeInput;
pub use data::{ConfigData, ValidatedConfigData};
mod error;
mod manager;
mod storage;

pub use error::*;
pub use manager::{ConfigManager, Manager};

#[cfg(test)]
pub use manager::MockManager;

use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.yaml";

pub fn init(dir: PathBuf) -> Result<ConfigManager, ConfigError> {
    let path = dir.join(CONFIG_FILE_NAME);
    let storage = storage::init(path)?;
    ConfigManager::new(storage)
}
