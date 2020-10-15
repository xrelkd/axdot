use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::error::{self, Error};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SymbolLink {
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub commands: Vec<Vec<String>>,
    pub directories: Vec<PathBuf>,
    pub empty_files: Vec<PathBuf>,
    pub links: HashMap<PathBuf, PathBuf>,
    pub copys: HashMap<PathBuf, PathBuf>,
}

impl Config {
    #[inline]
    pub fn from_str(s: &str) -> Result<Config, Error> {
        let content = serde_yaml::from_str(s).context(error::ParseYamlConfig)?;
        Ok(content)
    }

    #[inline]
    pub fn load<P: AsRef<Path>>(config_file: P) -> Result<Config, Error> {
        let config_file = config_file.as_ref();
        let data = std::fs::read_to_string(config_file)
            .context(error::ReadConfigFile { file_path: config_file.to_owned() })?;
        Ok(Self::from_str(&data)?)
    }
}
