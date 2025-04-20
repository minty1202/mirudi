mod core;
mod handler;
mod prompt_input;

pub use core::ScopeCommand;
use std::io::Error;

use crate::config::Manager;
use crate::git::GitOperations;
use handler::{DepsBuilder, HandleBuilder};

pub fn handle_scope(
    cmd: ScopeCommand,
    config: &mut dyn Manager,
    git: &dyn GitOperations,
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
    use crate::git::core::MockGitOperations;

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
        let mut git = MockGitOperations::new();

        config.expect_load().returning(|| Ok(ConfigData::default()));
        config.expect_save().returning(|_| Ok(()));

        git.expect_get_current_branch()
            .returning(|| Ok("test_branch".to_string()));

        let result = handle_scope(cmd, &mut config, &git);
        assert!(result.is_ok());
    }
}
