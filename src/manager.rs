use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::context::Context;
use crate::error::Error;

pub struct Manager {
    directories: Vec<PathBuf>,
    empty_files: Vec<PathBuf>,
    links: HashMap<PathBuf, PathBuf>,
    copys: HashMap<PathBuf, PathBuf>,
    commands: Vec<Vec<String>>,
}

impl From<Config> for Manager {
    fn from(config: Config) -> Manager {
        Manager {
            directories: config.directories,
            empty_files: config.empty_files,
            links: config.links,
            copys: config.copys,
            commands: config.commands,
        }
    }
}

impl Manager {
    #[allow(dead_code)]
    pub fn new(
        directories: Vec<PathBuf>,
        empty_files: Vec<PathBuf>,
        links: HashMap<PathBuf, PathBuf>,
        copys: HashMap<PathBuf, PathBuf>,
        commands: Vec<Vec<String>>,
    ) -> Manager {
        Manager {
            directories,
            empty_files,
            links,
            copys,
            commands,
        }
    }

    pub fn apply(&self, dry: bool, replace: bool, context: &Context) -> Result<(), Error> {
        for dir in &self.directories {
            helpers::create_directory(dry, context, dir)?;
        }

        for file in &self.empty_files {
            helpers::create_file(dry, replace, context, file)?;
        }

        for (src, dest) in &self.links {
            helpers::create_symlink(dry, replace, context, src, dest)?;
        }

        for (src, dest) in &self.copys {
            helpers::copy(dry, replace, context, src, dest)?;
        }

        for cmd in &self.commands {
            let (program, args) = match cmd.split_first() {
                Some((prog, args)) => (prog, args),
                None => return Err(Error::NoCommandProvided),
            };
            helpers::execute_command(dry, program, args)?;
        }

        Ok(())
    }
}

mod helpers {
    use super::*;

    pub fn ask_user(prompt: &str) -> Result<bool, Error> {
        use std::io::BufRead;
        let stdin = std::io::stdin();

        println!("{}", prompt);
        for line in stdin.lock().lines() {
            match line?.trim().to_lowercase().as_ref() {
                "yes" | "y" => return Ok(true),
                "no" | "n" => return Ok(false),
                _ => {
                    eprintln!("Enter a correct choice.");
                    println!("{}", prompt);
                    continue;
                }
            }
        }

        Err(Error::StandardInput)
    }

    pub fn copy<S: AsRef<Path>, D: AsRef<Path>>(
        dry: bool,
        replace: bool,
        context: &Context,
        src: &S,
        dest: &D,
    ) -> Result<(), Error> {
        let src = context.apply_path(src).canonicalize()?;
        let dest = context.apply_path(dest);

        if dest.exists() {
            if replace || helpers::ask_user(&format!("{:?} exists, delete it? [Y/n]", dest))? {
                println!("Removing {:?}", dest);
                helpers::remove_all(dry, &dest)?;
            } else {
                return Ok(());
            }
        }

        println!("Copying {:?} -> {:?}", src, dest);
        if dry {
            return Ok(());
        }

        if src.is_file() {
            std::fs::copy(src, dest)?;
        } else {
            std::fs::create_dir_all(&src)?;

            let options = fs_extra::dir::CopyOptions {
                overwrite: true,
                skip_exist: true,
                buffer_size: 64000,
                copy_inside: true,
                depth: 0,
            };
            fs_extra::dir::copy(src, dest, &options)?;
        }

        Ok(())
    }

    pub fn create_symlink<S: AsRef<Path>, D: AsRef<Path>>(
        dry: bool,
        replace: bool,
        context: &Context,
        src: &S,
        dest: &D,
    ) -> std::result::Result<(), Error> {
        let src = context.apply_path(src).canonicalize()?;
        let dest = context.apply_path(dest);

        println!("Linking {:?} -> {:?}", dest, src);

        if dry {
            return Ok(());
        }

        match dest.read_link() {
            Ok(dest) if dest == src => {
                println!("Skipping existing {:?} -> {:?}", dest, src);
                return Ok(());
            }
            Ok(dest) => {
                if replace || helpers::ask_user(&format!("{:?} exists, delete it? [Y/n]", dest))? {
                    helpers::remove_all(dry, &dest)?;
                }
            }
            Err(_err) => {}
        }

        Ok(std::os::unix::fs::symlink(src, dest)?)
    }

    pub fn create_directory<P: AsRef<Path>>(
        dry: bool,
        context: &Context,
        path: P,
    ) -> Result<(), Error> {
        let path = context.apply_path(path);

        println!("Creating {:?}", path);
        if dry {
            return Ok(());
        }

        if path.is_dir() {
            println!("Skipping existing {:?}", path);
        } else {
            std::fs::create_dir_all(path)?;
        }

        Ok(())
    }

    pub fn create_empty_file<P: AsRef<Path>>(dry: bool, path: P) -> Result<(), Error> {
        println!("Creating empty file {:?}", path.as_ref().to_string_lossy());
        if dry {
            return Ok(());
        }

        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        Ok(())
    }

    pub fn create_file<P: AsRef<Path>>(
        dry: bool,
        replace: bool,
        context: &Context,
        path: P,
    ) -> Result<(), Error> {
        let path = context.apply_path(&path);
        let dir_path = path.parent().unwrap();

        if path.exists() {
            if replace || helpers::ask_user(&format!("{:?} exist, delete it? [Y/n]", &path))? {
                helpers::remove_all(dry, &path)?;
            }
        }

        helpers::create_directory(dry, context, &dir_path)?;
        helpers::create_empty_file(dry, &path)?;
        Ok(())
    }

    pub fn remove_all<P: AsRef<Path>>(dry: bool, path: P) -> Result<(), Error> {
        if dry {
            return Ok(());
        }

        let path = path.as_ref();
        if path.is_file() || path.read_link().is_ok() {
            std::fs::remove_file(path)?;
        } else {
            std::fs::remove_dir_all(path)?;
        }

        Ok(())
    }

    pub fn execute_command(dry: bool, command: &str, args: &[String]) -> Result<(), Error> {
        println!("Executing \"{} {}\"", command, args.join(" "));

        if dry {
            return Ok(());
        }

        std::process::Command::new(command)
            .args(args)
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
