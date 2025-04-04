mod filesystem;
mod serializer;
mod storage;

use filesystem::OsFileSystem;
use serializer::YamlSerializer;
use storage::ConfigStorage;
use crate::config::error::ConfigError;

pub use storage::{Storage, DefaultConfigStorage};
#[cfg(test)]
pub use storage::MockStorage;

use std::path::PathBuf;
use std::sync::Arc;

pub fn init (path: PathBuf) -> Result<DefaultConfigStorage, ConfigError> {
    let fs = Arc::new(OsFileSystem::new());
    let serializer = Arc::new(YamlSerializer::new());
    let storage = ConfigStorage::new(path, fs, serializer)?;
    Ok(storage)
}
