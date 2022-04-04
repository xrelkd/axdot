use std::{io::Write, path::PathBuf};

use clap::{IntoApp, Parser, Subcommand};
use clap_complete::Shell;

use crate::{config::Config, context::Context, error::Result, manager::Manager};

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

impl Default for Cli {
    #[inline]
    fn default() -> Cli { Self::parse() }
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let context = if self.commands.is_standalone() { None } else { Some(Context::from_env()?) };

        match (self.commands, context) {
            (Commands::Version, _) => {
                let mut stdout = std::io::stdout();
                stdout
                    .write_all(Self::command().render_long_version().as_bytes())
                    .expect("failed to write to stdout");
                Ok(())
            }
            (Commands::Completions { shell }, _) => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                Ok(())
            }
            (Commands::Init, _) => {
                let config = Config::default();
                println!("{}", serde_yaml::to_string(&config).expect("Config is serializable"));
                Ok(())
            }
            (Commands::Apply { replace, config }, Some(context)) => {
                let manager = Self::create_manager(config)?;
                manager.apply(false, replace, &context)
            }
            (Commands::DryApply { replace, config }, Some(context)) => {
                let manager = Self::create_manager(config)?;
                manager.apply(true, replace, &context)
            }
            (_, None) => Ok(()),
        }
    }

    fn create_manager(config: Option<PathBuf>) -> Result<Manager> {
        let app_name = Self::command().get_name().to_string();

        let config_file = config.unwrap_or_else(|| PathBuf::from(format!("{}.yaml", app_name)));

        let config = Config::load(config_file)?;

        Ok(Manager::from(config))
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Shows current version")]
    Version,

    #[clap(about = "Shows shell completions")]
    Completions { shell: Shell },

    #[clap(about = "Generates empty configuration")]
    Init,

    #[clap(about = "Applies from configuration file")]
    Apply {
        #[clap(long = "config", help = "Configuration file path")]
        config: Option<PathBuf>,

        #[clap(
            long = "replace",
            short = 'r',
            help = "Replaces files/folders if they already exist"
        )]
        replace: bool,
    },

    #[clap(about = "Shows what would be applied")]
    DryApply {
        #[clap(long = "config", help = "Configuration file path")]
        config: Option<PathBuf>,

        #[clap(
            long = "replace",
            short = 'r',
            help = "Replaces files/folders if they already exist"
        )]
        replace: bool,
    },
}

impl Commands {
    #[inline]
    pub fn is_standalone(&self) -> bool { matches!(self, Self::Version | Self::Completions { .. }) }
}
