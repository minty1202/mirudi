#[derive(Debug, PartialEq)]
pub enum GitError {
    EmptyBranchName,
    NotGitManaged,
    FileNotFound,
    InvalidObjectType,
    InvalidUtf8,
    DiffExtractionFailed,
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::EmptyBranchName => write!(f, "ブランチ名が空です"),
            GitError::NotGitManaged => write!(f, "Git 管理されていないディレクトリです"),
            GitError::FileNotFound => write!(f, "ファイルが見つかりません"),
            GitError::InvalidObjectType => write!(f, "無効なオブジェクトタイプです"),
            GitError::InvalidUtf8 => write!(f, "無効な UTF-8 文字列です"),
            GitError::DiffExtractionFailed => write!(f, "差分の抽出に失敗しました"),
        }
    }
}

impl std::error::Error for GitError {}
