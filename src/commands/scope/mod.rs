use clap::Args;
use std::io::Error;

use crate::config::Manager;

#[derive(Args)]
pub struct ScopeCommand {
    #[arg(short, long)]
    pub current: bool,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(short, long)]
    pub old: Option<String>,

    #[arg(short, long)]
    pub new: Option<String>,

    #[arg(short, long)]
    pub path: Option<String>,
}

pub fn handle_scope(cmd: ScopeCommand, _config: &mut dyn Manager) -> Result<(), Error> {
    todo!("Scope コマンドはこれから実装する")
}
