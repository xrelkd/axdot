mod command;
mod config;
mod context;
mod error;
mod manager;

use self::{command::Command, error::Error};

fn main() -> Result<(), Error> { Command::new().run() }
