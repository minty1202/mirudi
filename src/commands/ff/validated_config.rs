use crate::config::{Manager, ValidatedConfigData};
use std::io::Error;

pub fn load(config: &mut dyn Manager) -> Result<ValidatedConfigData, Error> {
    let data = config.load().map_err(|e| {
        Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load config: {}", e),
        )
    })?;

    let data: ValidatedConfigData = data.try_into().map_err(|e| {
        Error::new(
            std::io::ErrorKind::Other,
            format!("未設定のフィールドがあります: {}", e),
        )
    })?;

    Ok(data)
}
