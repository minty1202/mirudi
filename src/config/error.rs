use std::io;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    EmptyBranchName,
    MissingField(String),
    Yaml(String),
    IoKind(io::ErrorKind),
    #[cfg(test)]
    Test,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EmptyBranchName => write!(f, "ブランチ名が空です"),
            ConfigError::MissingField(field) => {
                write!(f, "{} フィールドが入力されていません", field)
            }
            ConfigError::Yaml(err) => write!(f, "YAML エラー: {}", err),
            ConfigError::IoKind(err) => write!(f, "IO エラー: {}", err),
            #[cfg(test)]
            ConfigError::Test => write!(f, "テスト用エラー"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Yaml(err.to_string())
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoKind(err.kind())
    }
}
