pub mod init;
pub use init::InitCommand;
pub mod ff;
pub use ff::FFCommand;
pub mod handler;
pub use handler::CommandHandler;
mod scope;
pub use scope::ScopeCommand;
mod web;
pub use web::WebCommand;

use crate::config::Manager;
use crate::git::GitOperations;

use clap::Subcommand;
use std::io::Error;

#[derive(Subcommand)]
pub enum Commands {
    FF(FFCommand),
    Init(InitCommand),
    #[command(alias = "sc")]
    Scope(ScopeCommand),
    Web(WebCommand),
}

pub fn handle_command(
    handler: &dyn CommandHandler,
    command: Commands,
    config: &mut dyn Manager,
    git: &dyn GitOperations,
) -> Result<(), Error> {
    match command {
        Commands::FF(cmd) => handler.handle_ff(cmd, config, git),
        Commands::Init(cmd) => handler.handle_init(cmd, config),
        Commands::Scope(cmd) => handler.handle_scope(cmd, config, git),
        Commands::Web(cmd) => handler.handle_web(cmd),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use crate::commands::ff::{DiffMode, ScopeCommandInput};
    use crate::config::{Manager, MockManager};
    use crate::git::core::{MockGitOperations, SourceKind};

    struct MockCommandHandler {
        ff_called: RefCell<bool>,
        init_called: RefCell<bool>,
        scope_called: RefCell<bool>,
        ff_target: RefCell<Option<FFCommand>>,
        init_base: RefCell<Option<String>>,
        scope_target: RefCell<Option<ScopeCommand>>,
    }

    impl MockCommandHandler {
        fn new() -> Self {
            Self {
                ff_called: RefCell::new(false),
                init_called: RefCell::new(false),
                scope_called: RefCell::new(false),
                ff_target: RefCell::new(None),
                init_base: RefCell::new(None),
                scope_target: RefCell::new(None),
            }
        }
    }

    impl CommandHandler for MockCommandHandler {
        fn handle_ff(
            &self,
            cmd: FFCommand,
            _config: &mut dyn Manager,
            _git: &dyn GitOperations,
        ) -> Result<(), Error> {
            *self.ff_called.borrow_mut() = true;
            *self.ff_target.borrow_mut() = Some(cmd);
            Ok(())
        }

        fn handle_init(
            &self,
            command: InitCommand,
            _config: &mut dyn Manager,
        ) -> Result<(), Error> {
            *self.init_called.borrow_mut() = true;
            *self.init_base.borrow_mut() = command.base;
            Ok(())
        }

        fn handle_scope(
            &self,
            command: ScopeCommand,
            _config: &mut dyn Manager,
            _git: &dyn GitOperations,
        ) -> Result<(), Error> {
            *self.scope_called.borrow_mut() = true;
            *self.scope_target.borrow_mut() = Some(command);
            Ok(())
        }
        fn handle_web(&self, _cmd: WebCommand) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn test_handle_ff_command() {
        let handler = MockCommandHandler::new();
        let mut config = MockManager::new();
        let git = MockGitOperations::new();

        let command = Commands::FF(FFCommand {
            scope: ScopeCommandInput {
                current: true,
                branch: Some("test_branch".to_string()),
                old_path: Some("old_path".to_string()),
                new_path: Some("new_path".to_string()),
                path: None,
            },
            source: SourceKind::Commit,
            old_range: "1-10".to_string(),
            new_range: "11-20".to_string(),
            mode: DiffMode::Lines,
        });

        let result = handle_command(&handler, command, &mut config, &git);

        assert!(result.is_ok());
        assert!(*handler.ff_called.borrow());
    }

    #[test]
    fn test_handle_init_command_with_base() {
        let handler = MockCommandHandler::new();
        let mut config = MockManager::new();
        let git = MockGitOperations::new();
        let base = "test_branch".to_string();
        let command = Commands::Init(InitCommand {
            base: Some(base.clone()),
        });

        let result = handle_command(&handler, command, &mut config, &git);

        assert!(result.is_ok());
        assert!(*handler.init_called.borrow());
        assert_eq!(handler.init_base.borrow().as_ref(), Some(&base)); // 引数を検証
    }

    #[test]
    fn test_handle_scope_command() {
        let handler = MockCommandHandler::new();
        let mut config = MockManager::new();
        let git = MockGitOperations::new();
        let command = Commands::Scope(ScopeCommand {
            current: true,
            branch: Some("test_branch".to_string()),
            old: Some("old_path".to_string()),
            new: Some("new_path".to_string()),
            path: Some("test_path".to_string()),
        });
        let result = handle_command(&handler, command, &mut config, &git);
        assert!(result.is_ok());
        assert!(*handler.scope_called.borrow());
        assert_eq!(
            handler.scope_target.borrow().as_ref(),
            Some(&ScopeCommand {
                current: true,
                branch: Some("test_branch".to_string()),
                old: Some("old_path".to_string()),
                new: Some("new_path".to_string()),
                path: Some("test_path".to_string()),
            })
        );
    }
}
