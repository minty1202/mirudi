pub mod core;
pub mod error;

use core::{GitProvider, GitWebProvider};
pub use core::{Git, GitWeb, GitOperations};

pub fn init() -> Result<Git, error::GitError> {
    let git = Git::new(GitProvider::new());
    git.is_managed()?;
    Ok(git)
}

pub fn init_web() -> Result<GitWeb, error::GitError> {
    let git = GitWeb::new(GitWebProvider::new());
    Ok(git)
}
