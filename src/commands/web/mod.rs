use crate::config::Manager;
use crate::git::core::GitProvider;

use clap::Args;
use std::sync::Arc;
mod assets;
mod server;
use crate::commands::error::CommandError;

#[derive(Args)]
pub struct WebCommand {
    #[arg(short, long, default_value = "3210")]
    pub port: u16,
}

pub fn handle(
    cmd: WebCommand,
    config: &mut dyn Manager,
    git: Arc<dyn GitProvider + Send + Sync>,
) -> Result<(), CommandError> {
    println!("Webサーバーを起動しています。ポート: {}", cmd.port);

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        if let Err(e) = server::start_server(cmd.port, config, git).await {
            return Err(CommandError::WebServerError(e.to_string()));
        }
        Ok(())
    })
}
