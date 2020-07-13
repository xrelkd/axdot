use std::path::PathBuf;

use structopt::{clap::Shell, StructOpt};

use crate::{config::Config, context::Context, error::Error, manager::Manager};

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl Command {
    pub fn new() -> Command { Command::from_args() }

    #[inline]
    pub fn app_name() -> String { Command::clap().get_name().to_owned() }

    pub fn run(self) -> Result<(), Error> {
        let context =
            if self.subcommand.is_standalone() { None } else { Some(Context::from_env()?) };
        self.subcommand.run(context)
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(about = "Shows current version")]
    Version,

    #[structopt(about = "Shows shell completions")]
    Completions { shell: Shell },

    #[structopt(about = "Generates empty configuration")]
    Init,

    #[structopt(about = "Applies from configuration file")]
    Apply {
        #[structopt(long = "config", help = "Configuration file path")]
        config: Option<PathBuf>,

        #[structopt(
            long = "replace",
            short = "r",
            help = "Replaces files/folders if they already exist"
        )]
        replace: bool,
    },

    #[structopt(about = "Shows what would be applied")]
    DryApply {
        #[structopt(long = "config", help = "Configuration file path")]
        config: Option<PathBuf>,

        #[structopt(
            long = "replace",
            short = "r",
            help = "Replaces files/folders if they already exist"
        )]
        replace: bool,
    },
}

impl SubCommand {
    #[inline]
    pub fn is_standalone(&self) -> bool {
        match self {
            SubCommand::Version | SubCommand::Completions { .. } => true,
            _ => false,
        }
    }

    fn create_manager(config: Option<PathBuf>) -> Result<Manager, Error> {
        let config_file = config.unwrap_or(PathBuf::from(format!("{}.yaml", Command::app_name())));
        Ok(Config::load(config_file)?.into())
    }

    pub fn run(self, context: Option<Context>) -> Result<(), Error> {
        match (self, context) {
            (SubCommand::Version, _) => {
                Command::clap()
                    .write_version(&mut std::io::stdout())
                    .expect("failed to print version");
                Ok(())
            }
            (SubCommand::Completions { shell }, _) => {
                let app_name = Command::app_name();
                Command::clap().gen_completions_to(app_name, shell, &mut std::io::stdout());
                Ok(())
            }
            (SubCommand::Init, _) => {
                let config = Config::default();
                println!("{}", serde_yaml::to_string(&config).expect("Config is serializable"));
                Ok(())
            }
            (SubCommand::Apply { replace, config }, Some(context)) => {
                let manager = Self::create_manager(config)?;
                manager.apply(false, replace, &context)
            }
            (SubCommand::DryApply { replace, config }, Some(context)) => {
                let manager = Self::create_manager(config)?;
                manager.apply(true, replace, &context)
            }
            (_, None) => Ok(()),
        }
    }
}
