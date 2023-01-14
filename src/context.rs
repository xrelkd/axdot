use std::{
    fmt,
    path::{Path, PathBuf},
};

use snafu::ResultExt;

use crate::{error, error::Result};

#[derive(Debug, Clone)]
pub struct Context {
    pub user_name: String,
    pub home_dir: String,
}

impl Context {
    #[inline]
    pub fn from_env() -> Result<Self> {
        let user_name = std::env::var("USER").context(error::EnvUserNotFoundSnafu)?;
        let home_dir = std::env::var("HOME").context(error::EnvUserNotFoundSnafu)?;

        Ok(Self { user_name, home_dir })
    }

    #[inline]
    pub fn apply<S>(&self, s: S) -> String
    where
        S: fmt::Display,
    {
        s.to_string().replace("$USER", &self.user_name).replace("$HOME", &self.home_dir)
    }

    #[inline]
    pub fn apply_path<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        path.as_ref()
            .iter()
            .enumerate()
            .map(|(index, part)| {
                if index == 0 && part == "~" {
                    self.apply(part.to_string_lossy().replace('~', &self.home_dir))
                } else {
                    self.apply(part.to_string_lossy())
                }
            })
            .collect()
    }
}
