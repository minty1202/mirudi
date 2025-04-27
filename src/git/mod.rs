pub mod core;
mod error;

pub use core::*;
pub use error::*;

pub fn init() -> Result<Git, error::GitError> {
    let git = Git::new();
    git.is_managed()?;
    Ok(git)
}

pub fn init_web() -> Result<GitWeb, error::GitError> {
    let git = GitWeb::new();
    Ok(git)
}
