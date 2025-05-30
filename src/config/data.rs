use crate::config::error::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ConfigData {
    base_branch: Option<String>,
    current_branch: Option<String>,
    old_file_path: Option<String>,
    new_file_path: Option<String>,
}

pub struct ValidatedConfigData {
    base_branch: String,
    current_branch: String,
    old_file_path: String,
    new_file_path: String,
}

impl ValidatedConfigData {
    pub fn base_branch(&self) -> &String {
        &self.base_branch
    }

    pub fn current_branch(&self) -> &String {
        &self.current_branch
    }

    pub fn old_file_path(&self) -> &String {
        &self.old_file_path
    }

    pub fn new_file_path(&self) -> &String {
        &self.new_file_path
    }
}

pub struct ConfigScopeInput {
    pub current_branch: Option<String>,
    pub old_file_path: Option<String>,
    pub new_file_path: Option<String>,
}

impl ConfigData {
    pub fn base_branch(&self) -> Option<String> {
        self.base_branch.clone()
    }

    pub fn current_branch(&self) -> Option<String> {
        self.current_branch.clone()
    }

    pub fn old_file_path(&self) -> Option<String> {
        self.old_file_path.clone()
    }

    pub fn new_file_path(&self) -> Option<String> {
        self.new_file_path.clone()
    }

    pub fn set_base_branch(&mut self, branch: String) -> Result<(), ConfigError> {
        if branch.is_empty() {
            return Err(ConfigError::EmptyBranchName);
        }
        self.base_branch = Some(branch);
        Ok(())
    }

    pub fn set_scope(&mut self, scope: ConfigScopeInput) {
        self.current_branch = scope.current_branch.or(self.current_branch.clone());
        self.old_file_path = scope.old_file_path.or(self.old_file_path.clone());
        self.new_file_path = scope.new_file_path.or(self.new_file_path.clone());
    }
}

impl TryFrom<ConfigData> for ValidatedConfigData {
    type Error = ConfigError;

    fn try_from(config: ConfigData) -> Result<Self, Self::Error> {
        let base_branch = config
            .base_branch
            .ok_or(ConfigError::MissingField("base_branch".to_string()))?;
        let current_branch = config
            .current_branch
            .ok_or(ConfigError::MissingField("current_branch".to_string()))?;
        let old_file_path = config
            .old_file_path
            .ok_or(ConfigError::MissingField("old_file_path".to_string()))?;
        let new_file_path = config
            .new_file_path
            .ok_or(ConfigError::MissingField("new_file_path".to_string()))?;

        Ok(ValidatedConfigData {
            base_branch,
            current_branch,
            old_file_path,
            new_file_path,
        })
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
            current_branch: None,
            old_file_path: None,
            new_file_path: None,
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

    #[test]
    fn test_config_data_set_scope() {
        let mut config = ConfigData::default();
        let scope = ConfigScopeInput {
            current_branch: Some("feature".to_string()),
            old_file_path: Some("old_path".to_string()),
            new_file_path: Some("new_path".to_string()),
        };
        config.set_scope(scope);
        assert_eq!(config.current_branch(), Some("feature".to_string()));
        assert_eq!(config.old_file_path(), Some("old_path".to_string()));
        assert_eq!(config.new_file_path(), Some("new_path".to_string()));
    }
}
