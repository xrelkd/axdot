use std::path::{Path, PathBuf};

use crate::error::Error;

pub struct Context {
    pub user_name: String,
    pub home_dir: String,
}

impl Context {
    pub fn from_env() -> Result<Context, Error> {
        let user_name = match std::env::var("USER") {
            Ok(u) => u,
            Err(_) => return Err(Error::EnvUserNotFound),
        };

        let home_dir = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return Err(Error::EnvHomeNotFound),
        };

        Ok(Context { user_name, home_dir })
    }

    pub fn apply(&self, s: &str) -> String {
        s.replace("$USER", &self.user_name).replace("$HOME", &self.home_dir)
    }

    pub fn apply_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref()
            .iter()
            .enumerate()
            .map(|(index, part)| {
                if index == 0 && part == "~" {
                    self.apply(&part.to_string_lossy().replace("~", &self.home_dir))
                } else {
                    self.apply(&part.to_string_lossy())
                }
            })
            .collect()
    }
}
