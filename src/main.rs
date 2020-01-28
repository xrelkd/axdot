#[macro_use]
extern crate failure;

use structopt::StructOpt;

mod command;
mod config;
mod context;
mod error;
mod manager;

fn main() {
    use crate::command::Command;
    match Command::from_args().run() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
