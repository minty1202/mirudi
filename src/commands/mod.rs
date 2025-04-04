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

pub fn handle_command<H: CommandHandler<M>, M: Manager>(handler: &H, command: Commands, config: &M) -> Result<(), Error> {
    match command {
        Commands::FF(cmd) => handler.handle_ff(cmd.target, config),
        Commands::Init(cmd) => handler.handle_init(cmd, config),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use crate::config::{
        MockManager,
        data::ConfigData,
        error::ConfigError,
    };

    struct MockHandler {
        ff_called: RefCell<bool>,
        init_called: RefCell<bool>,
        ff_target: RefCell<Option<String>>, // FFCommand の引数を記録
        init_base: RefCell<Option<String>>, // InitCommand の引数を記録
    }
    
    impl MockHandler {
        fn new() -> Self {
            Self {
                ff_called: RefCell::new(false),
                init_called: RefCell::new(false),
                ff_target: RefCell::new(None),
                init_base: RefCell::new(None),
            }
        }
    }
    
    // MockManager を適切な型パラメータで使用
    impl CommandHandler<MockManager<ConfigData, ConfigError>> for MockHandler {
        fn handle_ff(&self, target: String, _config: &MockManager<ConfigData, ConfigError>) -> Result<(), Error> {
            *self.ff_called.borrow_mut() = true;
            *self.ff_target.borrow_mut() = Some(target); // 引数を記録
            Ok(())
        }
    
        fn handle_init(&self, cmd: InitCommand, _config: &MockManager<ConfigData, ConfigError>) -> Result<(), Error> {
            *self.init_called.borrow_mut() = true;
            *self.init_base.borrow_mut() = cmd.base; // 引数を記録
            Ok(())
        }
    }
    
    #[test]
    fn test_handle_command_ff_called() {
        let config = MockManager::new();
        let handler = MockHandler::new();
    
        let target = "test_target".to_string();
        handle_command(&handler, Commands::FF(FFCommand { target: target.clone() }), &config).unwrap();
    
        assert!(*handler.ff_called.borrow());
        assert_eq!(handler.ff_target.borrow().as_ref(), Some(&target)); // 引数を検証
    }
    
    #[test]
    fn test_handle_command_init_called() {
        let config = MockManager::new();
        let handler = MockHandler::new();
    
        let branch = "test_branch".to_string();
        handle_command(&handler, Commands::Init(InitCommand { base: Some(branch.clone()) }), &config).unwrap();
    
        assert!(*handler.init_called.borrow());
        assert_eq!(handler.init_base.borrow().as_ref(), Some(&branch)); // 引数を検証
    }
}
