use clap::Args;
use std::io::Error;
mod assets;
mod server;

#[derive(Args)]
pub struct WebCommand {
    #[arg(short, long, default_value = "8192")]
    pub port: u16,
}

pub fn handle_web(cmd: WebCommand) -> Result<(), Error> {
    println!("Webサーバーを起動しています。ポート: {}", cmd.port);

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        if let Err(e) = server::start_server(cmd.port).await {
            return Err(Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
        Ok(())
    })
}
