pub mod init;
pub use init::InitCommand;
pub mod ff;
pub use ff::FFCommand;
pub mod handler;
pub use handler::CommandHandler;

use crate::config::Manager;

use clap::Subcommand;
use std::io::Error;

#[derive(Subcommand)]
pub enum Commands {
    FF(FFCommand),
    Init(InitCommand),
}

pub fn handle_command(
    handler: &dyn CommandHandler,
    command: Commands,
    config: &mut dyn Manager,
) -> Result<(), Error> {
    match command {
        Commands::FF(cmd) => handler.handle_ff(cmd.target, config),
        Commands::Init(cmd) => handler.handle_init(cmd, config),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use crate::config::{Manager, MockManager};

    struct MockCommandHandler {
        ff_called: RefCell<bool>,
        init_called: RefCell<bool>,
        ff_target: RefCell<Option<String>>,
        init_base: RefCell<Option<String>>,
    }

    impl MockCommandHandler {
        fn new() -> Self {
            Self {
                ff_called: RefCell::new(false),
                init_called: RefCell::new(false),
                ff_target: RefCell::new(None),
                init_base: RefCell::new(None),
            }
        }
    }

    impl CommandHandler for MockCommandHandler {
        fn handle_ff(&self, target: String, _config: &mut dyn Manager) -> Result<(), Error> {
            *self.ff_called.borrow_mut() = true;
            *self.ff_target.borrow_mut() = Some(target);
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
    }

    #[test]
    fn test_handle_ff_command_with_target() {
        let handler = MockCommandHandler::new();
        let mut config = MockManager::new();
        let target = "test_target".to_string();
        let command = Commands::FF(FFCommand {
            target: target.clone(),
        });

        let result = handle_command(&handler, command, &mut config);

        assert!(result.is_ok());
        assert!(*handler.ff_called.borrow());
        assert_eq!(handler.ff_target.borrow().as_ref(), Some(&target));
        assert!(!*handler.init_called.borrow());
    }

    #[test]
    fn test_handle_init_command_with_base() {
        let handler = MockCommandHandler::new();
        let mut config = MockManager::new();
        let base = "test_branch".to_string();
        let command = Commands::Init(InitCommand {
            base: Some(base.clone()),
        });

        let result = handle_command(&handler, command, &mut config);

        assert!(result.is_ok());
        assert!(*handler.init_called.borrow());
        assert_eq!(handler.init_base.borrow().as_ref(), Some(&base)); // 引数を検証
        assert!(!*handler.ff_called.borrow());
    }
}
