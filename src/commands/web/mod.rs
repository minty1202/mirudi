use crate::config::Manager;
use crate::git::core::GitWeb;

use clap::Args;
use std::io::Error;
use std::sync::Arc;
mod assets;
mod server;

#[derive(Args)]
pub struct WebCommand {
    #[arg(short, long, default_value = "8192")]
    pub port: u16,
}

pub fn handle(
    cmd: WebCommand,
    config: &mut dyn Manager,
    git: GitWeb,
) -> Result<(), Error> {
    println!("Webサーバーを起動しています。ポート: {}", cmd.port);

    let web_git = Arc::new(git);

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        if let Err(e) = server::start_server(cmd.port, config, web_git).await {
            return Err(Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
        Ok(())
    })
}
