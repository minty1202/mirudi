use crate::commands::ff::{FFCommand, handle_ff};
use crate::commands::init::{InitCommand, handle_init};
use crate::commands::scope::{ScopeCommand, handle_scope};
use crate::config::Manager;
use crate::git::GitOperations;

use std::io::Error;

pub struct MirudiCommandHandler;

pub trait CommandHandler {
    fn handle_ff(
        &self,
        cmd: FFCommand,
        config: &mut dyn Manager,
        git: &dyn GitOperations,
    ) -> Result<(), Error>;
    fn handle_init(&self, cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error>;
    fn handle_scope(
        &self,
        cmd: crate::commands::scope::ScopeCommand,
        config: &mut dyn Manager,
        git: &dyn GitOperations,
    ) -> Result<(), Error>;
}

impl CommandHandler for MirudiCommandHandler {
    fn handle_ff(
        &self,
        cmd: FFCommand,
        config: &mut dyn Manager,
        git: &dyn GitOperations,
    ) -> Result<(), Error> {
        handle_ff(cmd, config, git)
    }

    fn handle_init(&self, cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error> {
        handle_init(cmd, config)
    }

    fn handle_scope(
        &self,
        cmd: ScopeCommand,
        config: &mut dyn Manager,
        git: &dyn GitOperations,
    ) -> Result<(), Error> {
        handle_scope(cmd, config, git)
    }
}
