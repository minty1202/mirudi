use crate::commands::init::{InitCommand, handle_init};
use crate::commands::ff::handle_ff;
use crate::config::{Manager, ConfigManager};

use std::io::Error;

pub struct MirudiCommandHandler;

pub trait CommandHandler<M: Manager> {
  fn handle_ff(&self, target: String, config: &M) -> Result<(), Error>;
  fn handle_init(&self, cmd: InitCommand, config: &M) -> Result<(), Error>;
}

impl <M: Manager> CommandHandler<M> for MirudiCommandHandler {
  fn handle_ff(&self, target: String, config: &M) -> Result<(), Error> {
    handle_ff(target, config)
  }

  fn handle_init(&self, cmd: InitCommand, config: &M) -> Result<(), Error> {
    handle_init(cmd, config)
  }
}
