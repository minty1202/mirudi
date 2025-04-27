use crate::config::{Manager, ValidatedConfigData};
use crate::commands::error::CommandError;

pub fn load(config: &mut dyn Manager) -> Result<ValidatedConfigData, CommandError> {
    let data = config.load()?;

    let data: ValidatedConfigData = data.try_into()?;

    Ok(data)
}
