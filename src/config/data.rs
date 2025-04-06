use crate::config::error::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ConfigData {
    base_branch: Option<String>,
}

impl ConfigData {
    // TODO: あとで使用する
    #[cfg(test)]
    pub fn base_branch(&self) -> Option<String> {
        self.base_branch.clone()
    }

    pub fn set_base_branch(&mut self, branch: String) -> Result<(), ConfigError> {
        if branch.is_empty() {
            return Err(ConfigError::EmptyBranchName);
        }
        self.base_branch = Some(branch);
        Ok(())
    }
}

#[cfg(test)]
impl Clone for ConfigData {
    fn clone(&self) -> Self {
        Self {
            base_branch: self.base_branch.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_data_default() {
        let config = ConfigData::default();
        assert_eq!(config.base_branch(), None);

        let config = ConfigData {
            base_branch: Some("main".to_string()),
        };
        assert_eq!(config.base_branch(), Some("main".to_string()));
    }

    #[test]
    fn test_config_data_set_base_branch() {
        let mut config = ConfigData::default();
        let branch = "main".to_string();
        config.set_base_branch(branch.clone()).unwrap();
        assert_eq!(config.base_branch(), Some(branch));

        let err = config.set_base_branch("".to_string());
        assert!(err.is_err());
        match err {
            Err(ConfigError::EmptyBranchName) => {}
            _ => panic!("Expected ConfigError::EmptyBranchName"),
        }
    }
}
