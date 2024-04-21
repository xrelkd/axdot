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
    #[serde(default)]
    pub commands: Vec<Vec<String>>,

    #[serde(default)]
    pub directories: Vec<PathBuf>,

    #[serde(default)]
    pub empty_files: Vec<PathBuf>,

    #[serde(default)]
    pub links: HashMap<PathBuf, PathBuf>,

    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::Config;

    #[test]
    fn test_parse_yaml() {
        let config_text = r"{}";
        let cfg = Config::from_yaml(config_text).unwrap();
        assert_eq!(cfg, Config::default());

        let config_text = r"
---
commands: []
directories: []
emptyFiles: []
links: {}
copies: {}
";

        let cfg = Config::from_yaml(config_text).unwrap();
        assert_eq!(cfg, Config::default());

        let config_text = r#"
---
commands:
    - ["ls"]
    - ["ls", "-a"]
directories: [ "doge", "meow" ]
emptyFiles:
    - "1"
    - "2"
    - 3
links:
    nvim: "~/.config/nvim"
copies:
    git/config: "~/.config/git/config"
"#;

        let cfg = Config::from_yaml(config_text).unwrap();
        assert_eq!(
            cfg,
            Config {
                commands: vec![vec!["ls".to_string()], vec!["ls".to_string(), "-a".to_string()]],
                directories: vec![PathBuf::from("doge"), PathBuf::from("meow")],
                empty_files: vec![PathBuf::from("1"), PathBuf::from("2"), PathBuf::from("3"),],
                links: {
                    let mut m = HashMap::default();
                    let _unused = m.insert(PathBuf::from("nvim"), PathBuf::from("~/.config/nvim"));
                    m
                },
                copies: {
                    let mut m = HashMap::default();
                    let _unused = m
                        .insert(PathBuf::from("git/config"), PathBuf::from("~/.config/git/config"));
                    m
                },
            }
        );
    }
}
