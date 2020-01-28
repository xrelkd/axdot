use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        Ok(serde_yaml::from_str(s)?)
    }

    #[inline]
    pub fn from_file<P: AsRef<Path>>(config_file: P) -> Result<Config, Error> {
        let data = std::fs::read_to_string(config_file)?;
        Ok(Self::from_str(&data)?)
    }
}
