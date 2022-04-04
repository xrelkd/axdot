mod command;
mod config;
mod context;
mod error;
mod manager;

use self::{command::Command, error::Result};

fn main() -> Result<()> { Command::new().run() }
