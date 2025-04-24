mod core;
mod filesystem;
mod serializer;

use crate::config::error::ConfigError;
use core::ConfigStorage;
use filesystem::OsFileSystem;
use serializer::YamlSerializer;

#[cfg(test)]
pub use core::MockStorage;
pub use core::{DefaultConfigStorage, Storage};

use std::path::PathBuf;
use std::sync::Arc;

pub fn init(path: PathBuf) -> Result<DefaultConfigStorage, ConfigError> {
    let fs = Arc::new(OsFileSystem::new());
    let serializer = Arc::new(YamlSerializer::new());
    let storage = ConfigStorage::new(path, fs, serializer)?;
    Ok(storage)
}
