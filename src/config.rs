use crate::utils::env::config_dir;

use once_cell::sync::Lazy;
use std::sync::RwLock;

use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    base_branch: Option<String>,
}

impl Config {
    pub const CONFIG_DIR: &'static str = ".mirudi";
    pub const CONFIG_FILE: &'static str = "config.yaml";

    pub fn save_base_branch(&mut self, new_branch: String) {
        let config_file_path = Self::file_path();
        self.base_branch = Some(new_branch);

        let content = serde_yaml::to_string(self).expect("YAML のシリアライズに失敗しました");
        let mut file = File::create(&config_file_path).expect("ファイルの作成に失敗しました");
        file.write_all(content.as_bytes())
            .expect("ファイルへの書き込みに失敗しました");
    }

    #[cfg(test)]
    pub fn base_branch(&self) -> Option<String> {
        self.base_branch.clone()
    }

    fn new() -> Self {
        Self::init_path();
        Self::load()
    }

    fn init_path() {
        let config_dir_path = Self::dir_path();
        if !config_dir_path.exists() {
            fs::create_dir_all(&config_dir_path).expect("ディレクトリの作成に失敗しました");
        }
        let config_file_path = Self::file_path();
        if !config_file_path.exists() {
            File::create(&config_file_path).expect("ファイルの作成に失敗しました");
            Self::write_default();
        }
    }

    fn load() -> Self {
        let config_file_path = Self::file_path();
    
        if !config_file_path.exists() {
            return Self::default();
        }

        let content = fs::read_to_string(&config_file_path)
            .expect("config.yaml の読み込みに失敗しました");
    
        match serde_yaml::from_str::<Self>(&content) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("config.yaml の形式が想定と異なります。default にフォールバックします。");
                Self::write_default();
                Self::default()
            }
        }
    }

    fn write_default() {
        let config_file = Self::file_path();
        let content = serde_yaml::to_string(&Self::default()).expect("YAML のシリアライズに失敗しました");
        let mut file = File::create(&config_file).expect("ファイルの作成に失敗しました");
        file.write_all(content.as_bytes())
            .expect("ファイルへの書き込みに失敗しました");
    }

    fn dir_path() -> PathBuf {
        config_dir().join(Self::CONFIG_DIR)
    }

    fn file_path() -> PathBuf {
        Self::dir_path().join(Self::CONFIG_FILE)
    }

    #[cfg(test)]
    fn reset(&self) {
        let config_file = Self::file_path();
        if config_file.exists() {
            fs::remove_file(config_file).expect("ファイルの削除に失敗しました");
        }
        Self::write_default();
    }
}

pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let config = Config::new();
    RwLock::new(config)
});

#[cfg(test)]
mod tests {
    use super::*;

    /*
        test_config_dir_path と test_config_file_path は
        プライベートなためメソッドをわざわざテストする必要はない。
        だが、ここが通過しないと開発中にホームディレクトリにファイルとディレクトリが作成されることになる。
        確認のためにテストを実行する。
     */

    #[test]
    fn test_config_dir_path() {
        let expected = config_dir().join(Config::CONFIG_DIR);
        let actual = Config::dir_path();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_config_file_path() {
        let expected = config_dir().join(Config::CONFIG_DIR).join(Config::CONFIG_FILE);
        let actual = Config::file_path();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_save_base_branch() {
        let mut config = CONFIG.write().expect("Config のロックに失敗しました");
        config.reset();

        let branch = String::from("test_branch");
        config.save_base_branch(branch.clone());

        let config_file = Config::file_path();
        let content = fs::read_to_string(config_file).expect("ファイルの読み込みに失敗しました");

        let loaded_config: Config = serde_yaml::from_str(&content).expect("YAML のデシリアライズに失敗しました");
        assert_eq!(loaded_config.base_branch, Some(branch.clone()));
        assert_eq!(config.base_branch, Some(branch));

    }
}
