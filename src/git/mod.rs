pub mod core;
pub mod error;

use core::GitProvider;
pub use core::{Git, GitOperations};

pub fn init() -> Result<Git, error::GitError> {
    let git = Git::new(GitProvider::new());
    git.is_managed()?;
    Ok(git)
}
