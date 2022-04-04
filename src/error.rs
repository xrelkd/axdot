use std::path::PathBuf;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("No command provided"))]
    NoCommandProvided,

    #[snafu(display("Failed to get `USER` name from environment variable, error: {source}"))]
    EnvUserNotFound { source: std::env::VarError },

    #[snafu(display("Failed to get `HOME` from environment variable, error: {source}"))]
    EnvHomeNotFound { source: std::env::VarError },

    #[snafu(display("Failed to read standard input, error: {source}"))]
    ReadStandardInput { source: std::io::Error },

    #[snafu(display("Could not read configuration file `{}`, error: {source}", file_path.display()))]
    ReadConfigFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not copy file `{}` -> `{}`, error: {source}",
            copy_source.display(), copy_destination.display()))]
    CopyDirectory {
        copy_source: PathBuf,
        copy_destination: PathBuf,
        source: fs_extra::error::Error,
    },

    #[snafu(display("Could not copy file `{}` -> `{}`, error: {source}",
            copy_source.display(), copy_destination.display()  ))]
    CopyFile { copy_source: PathBuf, copy_destination: PathBuf, source: std::io::Error },

    #[snafu(display("Could not remove file `{}`, error: {source}", file_path.display()))]
    RemoveFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not create directory `{}`, error: {source}", dir_path.display()))]
    CreateDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not remove directory `{}`, error: {source}", dir_path.display()))]
    RemoveDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(
        display(
            "Could not create symbol link `{}` -> `{}`, error: {source}",
            link_source.display(), link_destination.display()
        )
    )]
    CreateSymbolLink { link_source: PathBuf, link_destination: PathBuf, source: std::io::Error },

    #[snafu(display("Could not create empty file `{}`, error: {source}", file_path.display()))]
    CreateEmptyFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not canonicalize path `{}`, error: {source}", path.display()))]
    CanonicalizePath { path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not parse YAML configuration, error: {source}"))]
    ParseYamlConfig { source: serde_yaml::Error },

    #[snafu(display("Could not spawn external command `{command} {}`, error: {source}",  args.join(" ")))]
    SpawnExternalCommand { command: String, args: Vec<String>, source: std::io::Error },

    #[snafu(display("Could not wait for spawned process, error: {source}"))]
    WaitForSpawnedProcess { source: std::io::Error },
}
