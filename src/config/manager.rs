use crate::config::data::ConfigData;
use crate::config::error::ConfigError;
use crate::config::storage::{DefaultConfigStorage, Storage};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Manager<D: 'static = ConfigData, E: 'static = ConfigError> {
    fn save(&mut self, data: D) -> Result<(), E>;
    // TODO: あとで使用する
    #[allow(dead_code)]
    fn load(&mut self) -> Result<(), E>;
    fn get_default(&self) -> Result<D, E>;
}

pub struct ConfigManager<S: Storage = DefaultConfigStorage> {
    data: ConfigData,
    storage: S,
}

impl<S: Storage> ConfigManager<S> {
    pub fn new(storage: S) -> Result<Self, ConfigError> {
        let data = storage.load()?;

        Ok(Self { data, storage })
    }
}

impl<S: Storage> Manager for ConfigManager<S> {
    fn save(&mut self, data: ConfigData) -> Result<(), ConfigError> {
        self.storage.save(&data)?;
        self.data = data;
        Ok(())
    }

    fn load(&mut self) -> Result<(), ConfigError> {
        let data = self.storage.load()?;
        self.data = data;
        Ok(())
    }

    fn get_default(&self) -> Result<ConfigData, ConfigError> {
        Ok(ConfigData::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::error::ConfigError;
    use crate::config::storage::MockStorage;

    use std::io::{Error as IoError, ErrorKind as IoErrorKind};

    #[test]
    fn test_config_manager_get_default() {
        let mut mock_storage = MockStorage::new();

        mock_storage
            .expect_load()
            .returning(|| Ok(ConfigData::default()));

        let config_manager = ConfigManager::new(mock_storage).unwrap();
        let result = config_manager.get_default();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ConfigData::default());
    }

    mod config_storage_new {
        use super::*;

        #[test]
        fn test_returns_ok() {
            let mut mock_storage = MockStorage::new();

            mock_storage
                .expect_load()
                .returning(|| Ok(ConfigData::default()));

            let result = ConfigManager::new(mock_storage);
            assert!(result.is_ok());
        }

        #[test]
        fn test_returns_error() {
            let mut mock_storage = MockStorage::new();

            mock_storage.expect_load().returning(|| {
                Err(ConfigError::Io(IoError::new(
                    IoErrorKind::Other,
                    "テストエラー",
                )))
            });

            let result = ConfigManager::new(mock_storage);
            assert!(result.is_err());
        }

        #[test]
        fn test_load_once_called() {
            let mut mock_storage = MockStorage::new();

            mock_storage
                .expect_load()
                .once()
                .returning(|| Ok(ConfigData::default()));

            let _ = ConfigManager::new(mock_storage);
        }
    }

    mod config_storage_save {
        use super::*;

        #[test]
        fn test_returns_ok() {
            let mut mock_storage = MockStorage::new();

            mock_storage.expect_save().returning(|_| Ok(()));
            mock_storage
                .expect_load()
                .returning(|| Ok(ConfigData::default()));

            let mut config_manager = ConfigManager::new(mock_storage).unwrap();
            let data = ConfigData::default();
            let result = config_manager.save(data.clone());
            assert!(result.is_ok());
            assert_eq!(config_manager.data, data);
        }

        #[test]
        fn test_returns_error() {
            let mut mock_storage = MockStorage::new();

            mock_storage
                .expect_load()
                .returning(|| Ok(ConfigData::default()));

            mock_storage.expect_save().returning(|_| {
                Err(ConfigError::Io(IoError::new(
                    IoErrorKind::Other,
                    "テストエラー",
                )))
            });

            let mut config_manager = ConfigManager::new(mock_storage).unwrap();
            let mut data = ConfigData::default();
            data.set_base_branch("main".to_string()).unwrap();
            let result = config_manager.save(data.clone());
            assert!(result.is_err());
            assert_ne!(config_manager.data, data);
        }
    }

    mod config_storage_load {
        use super::*;

        #[test]
        fn test_returns_ok() {
            let mut mock_storage = MockStorage::new();

            mock_storage
                .expect_load()
                .returning(|| Ok(ConfigData::default()));

            let mut config_manager = ConfigManager::new(mock_storage).unwrap();
            let result = config_manager.load();
            assert!(result.is_ok());
        }

        #[test]
        fn test_returns_error() {
            let mut mock_storage = MockStorage::new();

            mock_storage
                .expect_load()
                .times(1)
                .returning(|| Ok(ConfigData::default()));

            mock_storage.expect_load().times(1).returning(|| {
                Err(ConfigError::Io(IoError::new(
                    IoErrorKind::Other,
                    "テストエラー",
                )))
            });

            let mut config_manager = ConfigManager::new(mock_storage).unwrap();
            let result = config_manager.load();
            assert!(result.is_err());
        }
    }
}
