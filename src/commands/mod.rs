pub mod init;
pub use init::InitCommand;
pub mod ff;
pub use ff::FFCommand;
mod scope;
pub use scope::ScopeCommand;
mod web;
pub use web::WebCommand;

use crate::config::Manager;
use crate::git::{GitOperations, core::GitWeb};

use clap::Subcommand;
use std::io::Error;

#[derive(Subcommand)]
pub enum CliCommands {
    FF(FFCommand),
    Init(InitCommand),
    #[command(alias = "sc")]
    Scope(ScopeCommand),
}

#[derive(Subcommand)]
pub enum ServerCommands {
    Web(WebCommand),
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(flatten)]
    Cli(CliCommands),

    #[command(flatten)]
    Server(ServerCommands),
}

pub fn handle_cli_command(
    command: CliCommands,
    config: &mut dyn Manager,
    git: &dyn GitOperations,
) -> Result<(), Error> {
    match command {
        CliCommands::FF(cmd) => ff::handle(cmd, config, git),
        CliCommands::Init(cmd) => init::handle(cmd, config),
        CliCommands::Scope(cmd) => scope::handle(cmd, config, git),
    }
}

pub fn handle_web_command(
    command: ServerCommands,
    config: &mut dyn Manager,
    git: GitWeb,
) -> Result<(), Error> {
    match command {
        ServerCommands::Web(cmd) => web::handle(cmd, config, git),
    }
}
