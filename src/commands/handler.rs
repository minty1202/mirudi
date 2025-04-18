use crate::commands::ff::handle_ff;
use crate::commands::init::{InitCommand, handle_init};
use crate::config::Manager;

use std::io::Error;

pub struct MirudiCommandHandler;

pub trait CommandHandler {
    fn handle_ff(&self, target: String, config: &mut dyn Manager) -> Result<(), Error>;
    fn handle_init(&self, cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error>;
}

impl CommandHandler for MirudiCommandHandler {
    fn handle_ff(&self, target: String, config: &mut dyn Manager) -> Result<(), Error> {
        handle_ff(target, config)
    }

    fn handle_init(&self, cmd: InitCommand, config: &mut dyn Manager) -> Result<(), Error> {
        handle_init(cmd, config)
    }
}
