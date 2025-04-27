use clap::Args;

use crate::commands::scope::{ScopeCommand, ScopeInputResolver, run_scope_silently};
use crate::commands::error::CommandError;

#[derive(Debug, Args, PartialEq)]
pub struct ScopeCommandInput {
    #[arg(short, long)]
    pub current: bool,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(long = "old-path")]
    pub old_path: Option<String>,

    #[arg(long = "new-path")]
    pub new_path: Option<String>,

    #[arg(short, long)]
    pub path: Option<String>,
}

impl ScopeCommandInput {
    pub fn to_scope_command(&self) -> ScopeCommand {
        ScopeCommand {
            current: self.current,
            branch: self.branch.clone(),
            old: self.old_path.clone(),
            new: self.new_path.clone(),
            path: self.path.clone(),
        }
    }

    pub fn resolve_scope_silently(
        &self,
        config: &mut dyn crate::config::Manager,
        git: &dyn crate::git::GitProvider,
    ) -> Result<(), CommandError> {
        let cmd = self.to_scope_command();
        if cmd.is_empty() {
            return Ok(());
        }

        run_scope_silently(cmd, config, git)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::MockManager;
    use crate::git::core::MockGitProvider;

    #[test]
    fn returns_ok() {
        let cmd = ScopeCommandInput {
            current: false,
            branch: None,
            old_path: None,
            new_path: None,
            path: None,
        };
        let mut config = MockManager::new();
        let git = MockGitProvider::new();

        let result = cmd.resolve_scope_silently(&mut config, &git);
        assert!(result.is_ok());
    }
}
