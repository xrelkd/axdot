use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

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
        let content =
            serde_yaml::from_str(s).map_err(|source| Error::ParseYamlConfig { source })?;
        Ok(content)
    }

    #[inline]
    pub fn load<P: AsRef<Path>>(config_file: P) -> Result<Config, Error> {
        let data = std::fs::read_to_string(&config_file).map_err(|source| {
            Error::ReadConfigFile { source, file_path: config_file.as_ref().to_owned() }
        })?;
        Ok(Self::from_str(&data)?)
    }
}
