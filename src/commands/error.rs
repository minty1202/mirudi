use crate::config::ConfigError;
use crate::git::GitError;
use std::io;

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Git(GitError),
    Config(ConfigError),
    IO(String),
    ArgParse(String),
    WebServerError(String),
    InvalidInput(String),
    InternalError(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Git(err) => write!(f, "Git エラー: {}", err),
            CommandError::Config(err) => write!(f, "Config エラー: {}", err),
            CommandError::IO(err) => write!(f, "IO エラー: {:?}", err),
            CommandError::ArgParse(err) => write!(f, "引数解析エラー: {}", err),
            CommandError::InvalidInput(err) => write!(f, "{}", err),
            CommandError::WebServerError(err) => write!(f, "Web サーバーエラー: {}", err),
            CommandError::InternalError(err) => write!(f, "内部エラー: {}", err),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<GitError> for CommandError {
    fn from(err: GitError) -> Self {
        CommandError::Git(err)
    }
}

impl From<ConfigError> for CommandError {
    fn from(err: ConfigError) -> Self {
        CommandError::Config(err)
    }
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        CommandError::IO(err.to_string())
    }
}
