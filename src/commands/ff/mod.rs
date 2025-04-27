mod handler;
mod mode;
mod range;
mod scope_input;
mod validated_config;

use crate::config::Manager;
use handler::DiffHandler;
pub use mode::DiffMode;
use range::Range;
pub use scope_input::ScopeCommandInput;

use crate::commands::error::CommandError;
use clap::Args;

use crate::git::{GitProvider, core::SourceKind};

#[derive(Args, Debug, PartialEq)]
pub struct FFCommand {
    #[command(flatten)]
    pub scope: ScopeCommandInput,

    #[arg(value_name = "OLD_FILE_RANGE")]
    pub old_range: String,

    #[arg(value_name = "NEW_FILE_RANGE")]
    pub new_range: String,

    #[arg(short, long, value_enum, default_value_t = SourceKind::Commit)]
    pub source: SourceKind,

    #[arg(short, long, value_enum, default_value_t = DiffMode::Slice)]
    pub mode: DiffMode,
}

pub fn handle(
    cmd: FFCommand,
    config: &mut dyn Manager,
    git: &dyn GitProvider,
) -> Result<(), CommandError> {
    cmd.scope.resolve_scope_silently(config, git)?;
    let data = validated_config::load(config)?;
    let mut handler = DiffHandler::build(cmd, git, data);
    handler.exec()?;
    Ok(())
}
