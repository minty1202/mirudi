use crate::config::storage::serializer::{Serializer, YamlSerializer};

use crate::config::{data::ConfigData, error::ConfigError};

use crate::config::storage::filesystem::{FileSystem, OsFileSystem};

use std::path::PathBuf;
use std::sync::Arc;

pub type DefaultConfigStorage = ConfigStorage<YamlSerializer, OsFileSystem>;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Storage<T = ConfigData, E = ConfigError> {
    fn save(&self, data: &T) -> Result<(), E>;
    fn load(&self) -> Result<T, E>;
}

pub struct ConfigStorage<S: Serializer, F: FileSystem> {
    file_path: PathBuf,
    fs: Arc<F>,
    serializer: Arc<S>,
}

impl<S: Serializer, F: FileSystem> ConfigStorage<S, F> {
    pub fn new(file_path: PathBuf, fs: Arc<F>, serializer: Arc<S>) -> Result<Self, ConfigError> {
        let storage = Self {
            file_path,
            fs,
            serializer,
        };
        storage.ensure_directory_exists()?;
        storage.ensure_file_exists()?;
        Ok(storage)
    }

    // TODO: あとで使用する
    #[cfg(test)]
    fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    fn ensure_directory_exists(&self) -> Result<(), ConfigError> {
        if let Some(dir) = self.file_path.parent() {
            if !self.fs.exists(dir) {
                self.fs.create_dir_all(dir)?;
            }
        }
        Ok(())
    }

    fn ensure_file_exists(&self) -> Result<(), ConfigError> {
        if !self.fs.exists(&self.file_path) {
            self.fs.create_file(&self.file_path)?;
        }
        Ok(())
    }
}

impl<S: Serializer, F: FileSystem> Storage for ConfigStorage<S, F> {
    fn save(&self, data: &ConfigData) -> Result<(), ConfigError> {
        let yaml = self.serializer.serialize(data)?;
        self.fs.write_file(&self.file_path, &yaml)?;
        Ok(())
    }

    fn load(&self) -> Result<ConfigData, ConfigError> {
        let content = self.fs.read_to_string(&self.file_path)?;
        let data: ConfigData = self.serializer.deserialize(&content)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::storage::filesystem::MockFileSystem;
    use crate::config::storage::serializer::MockSerializer;
    use serde::de::Error as SerdeError;

    const TEST_FILE_PATH: &str = "/test/dir/config.yaml";

    #[test]
    fn test_get_file_path() {
        let config_path = PathBuf::from(TEST_FILE_PATH);
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_exists().returning(|_| false);
        mock_fs.expect_create_dir_all().returning(|_| Ok(()));
        mock_fs.expect_create_file().returning(|_| Ok(()));
        let storage = ConfigStorage::new(
            config_path.clone(),
            Arc::new(mock_fs),
            Arc::new(MockSerializer::new()),
        )
        .unwrap();

        assert_eq!(storage.get_file_path(), &config_path);
    }

    mod config_storage_new {
        use super::*;

        #[test]
        fn test_config_storage_new_returns_ok() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();

            mock_fs.expect_exists().returning(|_| false);
            mock_fs.expect_create_dir_all().returning(|_| Ok(()));
            mock_fs.expect_create_file().returning(|_| Ok(()));
            let result = ConfigStorage::new(
                config_path,
                Arc::new(mock_fs),
                Arc::new(MockSerializer::new()),
            );

            assert!(result.is_ok());
        }

        #[test]
        fn test_config_storage_new_creates_directory() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let parent_dir = config_path.parent().unwrap().to_path_buf();

            let mut mock_fs = MockFileSystem::new();

            mock_fs.expect_exists().returning(|_| false);
            mock_fs
                .expect_create_dir_all()
                .with(mockall::predicate::eq(parent_dir))
                .times(1)
                .returning(|_| Ok(()));
            mock_fs.expect_create_file().returning(|_| Ok(()));

            let _ = ConfigStorage::new(
                config_path,
                Arc::new(mock_fs),
                Arc::new(MockSerializer::new()),
            );
        }

        #[test]
        fn test_config_storage_new_does_not_create_directory_if_exists() {
            let config_path = PathBuf::from(TEST_FILE_PATH);

            let mut mock_fs = MockFileSystem::new();

            mock_fs.expect_exists().returning(|_| true);
            mock_fs
                .expect_create_dir_all()
                .times(0)
                .returning(|_| Ok(()));
            mock_fs.expect_create_file().returning(|_| Ok(()));

            let _ = ConfigStorage::new(
                config_path,
                Arc::new(mock_fs),
                Arc::new(MockSerializer::new()),
            );
        }

        #[test]
        fn test_config_storage_new_creates_file() {
            let config_path = PathBuf::from(TEST_FILE_PATH);

            let mut mock_fs = MockFileSystem::new();

            mock_fs.expect_exists().returning(|_| false);
            mock_fs.expect_create_dir_all().returning(|_| Ok(()));
            mock_fs
                .expect_create_file()
                .with(mockall::predicate::eq(config_path.clone()))
                .times(1)
                .returning(|_| Ok(()));

            let _ = ConfigStorage::new(
                config_path,
                Arc::new(mock_fs),
                Arc::new(MockSerializer::new()),
            );
        }

        #[test]
        fn test_config_storage_new_does_not_create_file_if_exists() {
            let config_path = PathBuf::from(TEST_FILE_PATH);

            let mut mock_fs = MockFileSystem::new();

            mock_fs.expect_exists().returning(|_| true);
            mock_fs.expect_create_dir_all().returning(|_| Ok(()));
            mock_fs.expect_create_file().times(0).returning(|_| Ok(()));

            let _ = ConfigStorage::new(
                config_path,
                Arc::new(mock_fs),
                Arc::new(MockSerializer::new()),
            );
        }
    }

    mod config_storage_save {
        use super::*;

        #[test]
        fn test_config_storage_save() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();
            mock_fs.expect_exists().returning(|_| true);

            mock_serializer
                .expect_serialize()
                .with(mockall::predicate::always())
                .returning(|_: &ConfigData| Ok("base_branch: null\n".to_string()));

            mock_fs
                .expect_write_file()
                .with(
                    mockall::predicate::eq(config_path.clone()),
                    mockall::predicate::eq("base_branch: null\n"),
                )
                .returning(|_, _| Ok(()));

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let data = ConfigData::default();
            assert!(storage.save(&data).is_ok());
        }

        #[test]
        fn test_config_storage_save_invalid_data() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_serializer
                .expect_serialize::<ConfigData>()
                .returning(|_| Err(SerdeError::custom("Serialization error")));

            mock_fs.expect_write_file().times(0);

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let data = ConfigData::default();
            let result = storage.save(&data);
            assert!(result.is_err());
        }

        #[test]
        fn test_config_storage_save_file_system_error() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_serializer
                .expect_serialize()
                .with(mockall::predicate::always())
                .returning(|_: &ConfigData| Ok("base_branch: null\n".to_string()));

            mock_fs
                .expect_write_file()
                .with(
                    mockall::predicate::eq(config_path.clone()),
                    mockall::predicate::eq("base_branch: null\n"),
                )
                .returning(|_, _| {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "File system error",
                    ))
                });

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let data = ConfigData::default();
            let result = storage.save(&data);
            assert!(result.is_err());
        }
    }

    mod config_storage_load {
        use super::*;

        #[test]
        fn test_config_storage_load() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_fs
                .expect_read_to_string()
                .with(mockall::predicate::eq(config_path.clone()))
                .returning(|_| Ok("base_branch: null\n".to_string()));

            mock_serializer
                .expect_deserialize()
                .with(mockall::predicate::eq("base_branch: null\n"))
                .returning(|_| Ok(ConfigData::default()));

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let result = storage.load();
            assert!(result.is_ok());
            let data = result.unwrap();
            assert_eq!(data.base_branch(), None);
        }

        #[test]
        fn test_config_storage_load_has_value() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_fs
                .expect_read_to_string()
                .with(mockall::predicate::eq(config_path.clone()))
                .returning(|_| Ok("base_branch: main\n".to_string()));

            mock_serializer
                .expect_deserialize()
                .with(mockall::predicate::eq("base_branch: main\n"))
                .returning(|_| {
                    let mut data = ConfigData::default(); // ConfigData のインスタンスを作成
                    data.set_base_branch("main".to_string()).unwrap(); // base_branch を設定
                    Ok(data) // 設定済みのデータを返す
                });

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let result = storage.load();
            assert!(result.is_ok());
            let data = result.unwrap();
            assert_eq!(data.base_branch(), Some("main".to_string()));
        }

        #[test]
        fn test_config_storage_load_file_system_error() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_fs
                .expect_read_to_string()
                .with(mockall::predicate::eq(config_path.clone()))
                .returning(|_| {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "File system error",
                    ))
                });

            mock_serializer.expect_deserialize::<ConfigData>().times(0);

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let result = storage.load();

            assert!(result.is_err());
        }

        #[test]
        fn test_config_storage_load_deserialization_error() {
            let config_path = PathBuf::from(TEST_FILE_PATH);
            let mut mock_fs = MockFileSystem::new();
            let mut mock_serializer = MockSerializer::new();

            mock_fs.expect_exists().returning(|_| true);

            mock_fs
                .expect_read_to_string()
                .with(mockall::predicate::eq(config_path.clone()))
                .returning(|_| Ok("invalid_yaml".to_string()));

            mock_serializer
                .expect_deserialize::<ConfigData>()
                .with(mockall::predicate::eq("invalid_yaml"))
                .returning(|_| Err(SerdeError::custom("Deserialization error")));

            let storage =
                ConfigStorage::new(config_path, Arc::new(mock_fs), Arc::new(mock_serializer))
                    .unwrap();
            let result = storage.load();

            assert!(result.is_err());
        }
    }
}
