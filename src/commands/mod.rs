pub mod init;
pub use init::{InitCommand, handle_init};
pub mod ff;
pub use ff::handle_ff;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    FF {
        target: String,
    },
    Init(InitCommand),
}

pub fn handle_command(command: Commands) {
    match command {
        Commands::FF { target } => {
            handle_ff(target);
        }
        Commands::Init(cmd) => {
            handle_init(cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_command() {
        let target = "test_target".to_string();
        handle_command(Commands::FF {
            target: target.clone(),
        });
        assert_eq!(target, "test_target");

        let branch = "test_branch".to_string();
        handle_command(Commands::Init(InitCommand {
            base: Some(branch.clone()),
        }));
        assert_eq!(branch, "test_branch");
    }
}
