mod config;
mod diff;
mod git;
mod utils;

use clap::Parser;
mod commands;

use commands::{Commands, handle_command, handler::MirudiCommandHandler};

use std::process;

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
    let handler = MirudiCommandHandler;

    if let Err(e) = handle_command(&handler, cli.command, &mut config, &git) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}
