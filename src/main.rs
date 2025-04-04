mod config;
mod utils;

use clap::Parser;
mod commands;

use commands::{
    Commands,
    handler::MirudiCommandHandler,
    handle_command,
};

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
    let config = config::init(utils::env::config_dir()).unwrap_or_else(|err| {
        eprintln!("エラー: {}", err);
        process::exit(1);
    });
    let handler = MirudiCommandHandler;

    if let Err(e) = handle_command(&handler, cli.command, &config) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}
