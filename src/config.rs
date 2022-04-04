use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{error, error::Result};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SymbolLink {
    path: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub commands: Vec<Vec<String>>,
    pub directories: Vec<PathBuf>,
    pub empty_files: Vec<PathBuf>,
    pub links: HashMap<PathBuf, PathBuf>,
    pub copies: HashMap<PathBuf, PathBuf>,
}

impl Config {
    #[inline]
    pub fn from_yaml(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).context(error::ParseYamlConfigSnafu)
    }

    #[inline]
    pub fn load<P>(config_file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let config_file = config_file.as_ref();
        let data = std::fs::read_to_string(config_file)
            .context(error::ReadConfigFileSnafu { file_path: config_file.to_owned() })?;

        Self::from_yaml(&data)
    }
}
