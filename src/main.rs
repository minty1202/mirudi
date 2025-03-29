use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mirudi", version, about = "github の diff を簡単に行える CLI ツール")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    FF {
        target: String,
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::FF { target } => {
            println!("{}", target);
        }
    }
}
