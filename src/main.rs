mod command;
mod config;
mod context;
mod error;
mod manager;

use self::command::Cli;

fn main() {
    if let Err(err) = Cli::default().run() {
        eprintln!("{err}");
        std::process::exit(-1);
    }
}
