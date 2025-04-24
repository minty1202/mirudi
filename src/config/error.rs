#[derive(Debug)]
pub enum ConfigError {
    EmptyBranchName,
    MissingField(String),
    Yaml(serde_yaml::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EmptyBranchName => write!(f, "ブランチ名が空です"),
            ConfigError::MissingField(field) => {
                write!(f, "{} フィールドが入力されていません", field)
            }
            ConfigError::Yaml(err) => write!(f, "YAML エラー: {}", err),
            ConfigError::Io(err) => write!(f, "IO エラー: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Yaml(err)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}
