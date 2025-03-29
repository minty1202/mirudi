use clap::Parser;
mod commands;
use commands::Commands;
use commands::handle_command;

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
    handle_command(cli.command);
}
