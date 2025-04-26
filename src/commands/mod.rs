pub mod init;
pub use init::InitCommand;
pub mod ff;
pub use ff::FFCommand;
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
    command: Commands,
    config: &mut dyn Manager,
    git: &dyn GitOperations,
) -> Result<(), Error> {
    match command {
        Commands::FF(cmd) => ff::handle_ff(cmd, config, git),
        Commands::Init(cmd) => init::handle_init(cmd, config),
        Commands::Scope(cmd) => scope::handle_scope(cmd, config, git),
        Commands::Web(cmd) => web::handle_web(cmd, config, git),
    }
}
