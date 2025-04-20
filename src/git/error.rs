#[derive(Debug, PartialEq)]
pub enum GitError {
    EmptyBranchName,
    NotGitManaged,
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::EmptyBranchName => write!(f, "ブランチ名が空です"),
            GitError::NotGitManaged => write!(f, "Git 管理されていないディレクトリです"),
        }
    }
}

impl std::error::Error for GitError {}
