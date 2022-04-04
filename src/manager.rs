use std::{collections::HashMap, path::PathBuf};

use crate::{config::Config, context::Context, error::Error};

pub struct Manager {
    directories: Vec<PathBuf>,
    empty_files: Vec<PathBuf>,
    links: HashMap<PathBuf, PathBuf>,
    copies: HashMap<PathBuf, PathBuf>,
    commands: Vec<Vec<String>>,
}

impl From<Config> for Manager {
    fn from(Config { directories, empty_files, links, copies, commands }: Config) -> Self {
        Self { directories, empty_files, links, copies, commands }
    }
}

impl Manager {
    #[allow(dead_code)]
    pub fn new(
        directories: Vec<PathBuf>,
        empty_files: Vec<PathBuf>,
        links: HashMap<PathBuf, PathBuf>,
        copies: HashMap<PathBuf, PathBuf>,
        commands: Vec<Vec<String>>,
    ) -> Self {
        Self { directories, empty_files, links, copies, commands }
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

        for (src, dest) in &self.copies {
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
    use std::{fmt, path::Path};

    use snafu::ResultExt;

    use crate::{context::Context, error, error::Result};

    pub fn ask_user<S>(prompt: S) -> Result<bool>
    where
        S: fmt::Display,
    {
        use std::io::BufRead;
        let stdin = std::io::stdin();

        println!("{prompt}");
        for line in stdin.lock().lines() {
            let line = line.context(error::ReadStandardInputSnafu)?;
            match line.trim().to_lowercase().as_ref() {
                "yes" | "y" => return Ok(true),
                "no" | "n" => return Ok(false),
                _ => {
                    eprintln!("Enter a correct choice.");
                    println!("{prompt}");
                    continue;
                }
            }
        }

        Ok(false)
    }

    pub fn copy<S, D>(dry: bool, replace: bool, context: &Context, src: S, dest: D) -> Result<()>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        let copy_source = context
            .apply_path(&src)
            .canonicalize()
            .context(error::CanonicalizePathSnafu { path: src.as_ref().to_path_buf() })?;
        let copy_destination = context.apply_path(dest);

        let prompt = format!("`{}` exists, delete it? [Y/n]", copy_destination.display());
        if copy_destination.exists() {
            if replace || self::ask_user(&prompt)? {
                println!("Removing `{}`", copy_destination.display());
                self::remove_all(dry, &copy_destination)?;
            } else {
                return Ok(());
            }
        }

        println!("Copying `{}` -> `{}`", copy_source.display(), copy_destination.display());
        if dry {
            return Ok(());
        }

        if copy_source.is_file() {
            std::fs::copy(&copy_source, &copy_destination)
                .context(error::CopyFileSnafu { copy_source, copy_destination })?;
        } else {
            if let Some(dest_parent) = copy_destination.parent() {
                std::fs::create_dir_all(&dest_parent)
                    .context(error::CreateDirectorySnafu { dir_path: dest_parent.to_path_buf() })?;
            }

            let options = fs_extra::dir::CopyOptions {
                overwrite: true,
                skip_exist: true,
                buffer_size: 64000,
                copy_inside: true,
                content_only: false,
                depth: 0,
            };

            fs_extra::dir::copy(&copy_source, &copy_destination, &options)
                .context(error::CopyDirectorySnafu { copy_source, copy_destination })?;
        }

        Ok(())
    }

    pub fn create_symlink<S, D>(
        dry: bool,
        replace: bool,
        context: &Context,
        src: S,
        dest: D,
    ) -> Result<()>
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        let link_source = context
            .apply_path(&src)
            .canonicalize()
            .context(error::CanonicalizePathSnafu { path: src.as_ref().to_path_buf() })?;
        let link_destination = context.apply_path(dest);

        println!("Linking `{}` -> `{}`", link_destination.display(), link_source.display());

        if dry {
            return Ok(());
        }

        match link_destination.read_link() {
            Ok(dest) if dest == link_source => {
                println!("Skipping existing `{}` -> `{}`", dest.display(), link_source.display());
                return Ok(());
            }
            Ok(dest) => {
                let prompt = format!("`{}` exists, delete it? [Y/n]", dest.display());
                if replace || self::ask_user(&prompt)? {
                    self::remove_all(dry, &dest)?;
                }
            }
            Err(_err) => {}
        }

        std::os::unix::fs::symlink(&link_source, &link_destination)
            .context(error::CreateSymbolLinkSnafu { link_source, link_destination })?;

        Ok(())
    }

    pub fn create_directory<P>(dry: bool, context: &Context, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let dir_path = context.apply_path(path);

        println!("Creating `{}`", dir_path.display());
        if dry {
            return Ok(());
        }

        if dir_path.is_dir() {
            println!("Skipping existing `{}`", dir_path.display());
        } else {
            std::fs::create_dir_all(&dir_path).context(error::CreateDirectorySnafu { dir_path })?;
        }

        Ok(())
    }

    pub fn create_empty_file<P>(dry: bool, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file_path = path.as_ref();

        println!("Creating empty file `{}`", file_path.display());
        if dry {
            return Ok(());
        }

        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .context(error::CreateEmptyFileSnafu { file_path: file_path.to_owned() })?;

        Ok(())
    }

    pub fn create_file<P>(dry: bool, replace: bool, context: &Context, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = context.apply_path(&path);
        let dir_path = path.parent().unwrap();

        let prompt = format!("`{}` exist, delete it? [Y/n]", path.display());
        if path.exists() && (replace || self::ask_user(&prompt)?) {
            self::remove_all(dry, &path)?;
        }

        self::create_directory(dry, context, &dir_path)?;
        self::create_empty_file(dry, &path)?;

        Ok(())
    }

    pub fn remove_all<P>(dry: bool, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        if dry {
            return Ok(());
        }

        let path = path.as_ref();
        if path.is_file() || path.read_link().is_ok() {
            std::fs::remove_file(path)
                .context(error::RemoveFileSnafu { file_path: path.to_owned() })?;
        } else {
            std::fs::remove_dir_all(path)
                .context(error::RemoveDirectorySnafu { dir_path: path.to_owned() })?;
        }

        Ok(())
    }

    pub fn execute_command(dry: bool, command: &str, args: &[String]) -> Result<()> {
        println!("Executing `{command} {}`", args.join(" "));

        if dry {
            return Ok(());
        }

        std::process::Command::new(command)
            .args(args)
            .spawn()
            .context(error::SpawnExternalCommandSnafu {
                command: command.to_owned(),
                args: args.to_vec(),
            })?
            .wait_with_output()
            .context(error::WaitForSpawnedProcessSnafu)?;

        Ok(())
    }
}
