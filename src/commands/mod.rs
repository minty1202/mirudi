pub mod init;
pub use init::{InitSubcommand, handle_init};
pub mod ff;
pub use ff::handle_ff;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    FF {
        target: String,
    },
    Init {
        #[command(subcommand)]
        subcommand: InitSubcommand,
    },
}

pub fn handle_command(command: Commands) {
    match command {
        Commands::FF { target } => {
            handle_ff(target);
        }
        Commands::Init { subcommand } => {
            handle_init(subcommand);
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
        handle_command(Commands::Init {
            subcommand: InitSubcommand::Base {
                branch: branch.clone(),
            },
        });
        assert_eq!(branch, "test_branch");
    }
}
