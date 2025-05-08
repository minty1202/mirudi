mod commands;
mod config;
mod diff;
mod git;
mod utils;

use clap::Parser;
use commands::{Commands, handle_cli_command, handle_web_command};

use std::process;
use std::sync::Arc;

use git::GitProvider;

#[derive(Parser)]
#[command(
    name = "mirudi",
    version,
    about = "github の diff を簡単に行える CLI ツール"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    let mut config = config::init(utils::env::config_dir()).unwrap_or_else(|err| {
        eprintln!("エラー: {}", err);
        process::exit(1);
    });

    let git = git::init().unwrap_or_else(|err| {
        eprintln!("エラー: {}", err);
        process::exit(1);
    });

    match cli.command {
        Commands::Cli(command) => {
            if let Err(e) = handle_cli_command(command, &mut config, &git) {
                eprintln!("エラー: {}", e);
                process::exit(1);
            }
        }
        Commands::Server(command) => {
            let git: Arc<dyn GitProvider + Send + Sync> = Arc::new(git);

            if let Err(e) = handle_web_command(command, &mut config, git) {
                eprintln!("エラー: {}", e);
                process::exit(1);
            }
        }
    }
}
