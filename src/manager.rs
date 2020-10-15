use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use snafu::ResultExt;

use crate::{config::Config, context::Context, error, error::Error};

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
        Manager { directories, empty_files, links, copys, commands }
    }

    pub fn apply(&self, dry: bool, replace: bool, context: &Context) -> Result<(), Error> {
        for dir in &self.directories {
            helpers::create_directory(dry, context, dir)?;
        }

        for empty_file in &self.empty_files {
            helpers::create_file(dry, replace, context, empty_file)?;
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
            let line = line.context(error::ReadStandardInput)?;
            match line.trim().to_lowercase().as_ref() {
                "yes" | "y" => return Ok(true),
                "no" | "n" => return Ok(false),
                _ => {
                    eprintln!("Enter a correct choice.");
                    println!("{}", prompt);
                    continue;
                }
            }
        }

        Ok(false)
    }

    pub fn copy<S: AsRef<Path>, D: AsRef<Path>>(
        dry: bool,
        replace: bool,
        context: &Context,
        src: &S,
        dest: &D,
    ) -> Result<(), Error> {
        let copy_source = context
            .apply_path(src)
            .canonicalize()
            .context(error::CanonicalizePath { path: src.as_ref().to_path_buf() })?;
        let copy_destination = context.apply_path(dest);

        if copy_destination.exists() {
            if replace
                || helpers::ask_user(&format!(
                    "{} exists, delete it? [Y/n]",
                    copy_destination.display()
                ))?
            {
                println!("Removing {}", copy_destination.display());
                helpers::remove_all(dry, &copy_destination)?;
            } else {
                return Ok(());
            }
        }

        println!("Copying {} -> {}", copy_source.display(), copy_destination.display());
        if dry {
            return Ok(());
        }

        if copy_source.is_file() {
            std::fs::copy(&copy_source, &copy_destination)
                .context(error::CopyFile { copy_source, copy_destination })?;
        } else {
            if let Some(dest_parent) = copy_destination.parent() {
                std::fs::create_dir_all(&dest_parent)
                    .context(error::CreateDirectory { dir_path: dest_parent.to_path_buf() })?;
            }

            let options = fs_extra::dir::CopyOptions {
                overwrite: true,
                skip_exist: true,
                buffer_size: 64000,
                copy_inside: true,
                depth: 0,
            };

            fs_extra::dir::copy(&copy_source, &copy_destination, &options)
                .context(error::CopyDirectory { copy_source, copy_destination })?;
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
        let link_source = context
            .apply_path(src)
            .canonicalize()
            .context(error::CanonicalizePath { path: src.as_ref().to_path_buf() })?;
        let link_destination = context.apply_path(dest);

        println!("Linking {} -> {}", link_destination.display(), link_source.display());

        if dry {
            return Ok(());
        }

        match link_destination.read_link() {
            Ok(dest) if dest == link_source => {
                println!("Skipping existing {} -> {}", dest.display(), link_source.display());
                return Ok(());
            }
            Ok(dest) => {
                if replace
                    || helpers::ask_user(&format!("{} exists, delete it? [Y/n]", dest.display()))?
                {
                    helpers::remove_all(dry, &dest)?;
                }
            }
            Err(_err) => {}
        }

        Ok(std::os::unix::fs::symlink(&link_source, &link_destination)
            .context(error::CreateSymbolLink { link_source, link_destination })?)
    }

    pub fn create_directory<P: AsRef<Path>>(
        dry: bool,
        context: &Context,
        path: P,
    ) -> Result<(), Error> {
        let dir_path = context.apply_path(path);

        println!("Creating {}", dir_path.display());
        if dry {
            return Ok(());
        }

        if dir_path.is_dir() {
            println!("Skipping existing {}", dir_path.display());
        } else {
            std::fs::create_dir_all(&dir_path).context(error::CreateDirectory { dir_path })?;
        }

        Ok(())
    }

    pub fn create_empty_file<P: AsRef<Path>>(dry: bool, path: P) -> Result<(), Error> {
        let file_path = path.as_ref();

        println!("Creating empty file {}", file_path.display());
        if dry {
            return Ok(());
        }

        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .context(error::CreateEmptyFile { file_path: file_path.to_owned() })?;

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

        if path.exists()
            && (replace
                || helpers::ask_user(&format!("{} exist, delete it? [Y/n]", path.display()))?)
        {
            helpers::remove_all(dry, &path)?;
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
            std::fs::remove_file(path).context(error::RemoveFile { file_path: path.to_owned() })?;
        } else {
            std::fs::remove_dir_all(path)
                .context(error::RemoveDirectory { dir_path: path.to_owned() })?;
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
            .spawn()
            .context(error::SpawnExternalCommand {
                command: command.to_owned(),
                args: args.to_vec(),
            })?
            .wait_with_output()
            .context(error::WaitForSpawnedProcess)?;

        Ok(())
    }
}
