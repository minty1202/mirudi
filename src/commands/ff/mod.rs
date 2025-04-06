use crate::config::Manager;

use clap::Args;
use std::io::Error;

#[derive(Args)]
pub struct FFCommand {
    pub target: String,
}

pub fn handle_ff(target: String, _config: &mut dyn Manager) -> Result<(), Error> {
    println!("FF target: {}", target);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MockManager;

    #[test]
    fn test_returns_ok() {
        let target = "test_target".to_string();
        let mut mock_manager = MockManager::new();
        let result = handle_ff(target.clone(), &mut mock_manager);
        assert!(result.is_ok());
    }
}
