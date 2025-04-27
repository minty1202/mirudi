mod core;
pub mod handler;
mod prompt_input;

pub use core::{ScopeCommand, ScopeInputResolver};
use std::io::Error;

use crate::config::Manager;
use crate::git::GitProvider;
use handler::{DepsBuilder, HandleBuilder};

pub fn run_scope_silently(
    cmd: ScopeCommand,
    config: &mut dyn crate::config::Manager,
    git: &dyn crate::git::GitProvider,
) -> Result<(), std::io::Error> {
    let deps = DepsBuilder::new().git(git).build()?;
    let mut handler = HandleBuilder::new()
        .cmd(cmd)
        .config(config)
        .prompt_input(deps.prompt_input)
        .get_current_branch_name(deps.get_current_branch)
        .no_display(true)
        .build()?;

    handler.exec()?;
    Ok(())
}

pub fn handle(
    cmd: ScopeCommand,
    config: &mut dyn Manager,
    git: &dyn GitProvider,
) -> Result<(), Error> {
    let deps = DepsBuilder::new().git(git).build()?;
    let mut handler = HandleBuilder::new()
        .cmd(cmd)
        .config(config)
        .prompt_input(deps.prompt_input)
        .get_current_branch_name(deps.get_current_branch)
        .build()?;

    handler.exec()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::ConfigData;
    use crate::config::MockManager;
    use crate::git::core::MockGitProvider;

    #[test]
    fn returns_ok() {
        let cmd = ScopeCommand {
            current: true,
            branch: None,
            old: None,
            new: None,
            path: None,
        };
        let mut config = MockManager::new();
        let mut git = MockGitProvider::new();

        config.expect_load().returning(|| Ok(ConfigData::default()));
        config.expect_save().returning(|_| Ok(()));

        git.expect_get_current_branch()
            .returning(|| Ok("test_branch".to_string()));

        let result = handle(cmd, &mut config, &git);
        assert!(result.is_ok());
    }

    #[test]
    fn run_scope_silently_returns_ok() {
        let cmd = ScopeCommand {
            current: true,
            branch: None,
            old: None,
            new: None,
            path: None,
        };
        let mut config = MockManager::new();
        let mut git = MockGitProvider::new();

        config.expect_load().returning(|| Ok(ConfigData::default()));
        config.expect_save().returning(|_| Ok(()));

        git.expect_get_current_branch()
            .returning(|| Ok("test_branch".to_string()));
        let result = run_scope_silently(cmd, &mut config, &git);
        assert!(result.is_ok());
    }
}
