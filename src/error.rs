use std::path::PathBuf;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get USER name from environment variable"))]
    EnvUserNotFound,

    #[snafu(display("Failed to get HOME from environment variable"))]
    EnvHomeNotFound,

    #[snafu(display("no command provided"))]
    NoCommandProvided,

    #[snafu(display("Failed to read standard input, error: {}", source))]
    ReadStandardInput { source: std::io::Error },

    #[snafu(display("Could not read configuration file {}, error: {}", file_path.display(), source))]
    ReadConfigFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not copy file {} -> {}, error: {}",
            copy_source.display(), copy_destination.display(), source))]
    CopyDirectory {
        copy_source: PathBuf,
        copy_destination: PathBuf,
        source: fs_extra::error::Error,
    },

    #[snafu(display("Could not copy file {} -> {}, error: {}",
            copy_source.display(), copy_destination.display(), source))]
    CopyFile { copy_source: PathBuf, copy_destination: PathBuf, source: std::io::Error },

    #[snafu(display("Could not remove file {}, error: {}", file_path.display(), source))]
    RemoveFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not create directory {}, error: {}", dir_path.display(), source))]
    CreateDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not remove directory {}, error: {}", dir_path.display(), source))]
    RemoveDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not create symbol link {} -> {}, error: {}",
            link_source.display(), link_destination.display(), source))]
    CreateSymbolLink { source: std::io::Error, link_source: PathBuf, link_destination: PathBuf },

    #[snafu(display("Could not create empty file {}, error: {}", file_path.display(), source))]
    CreateEmptyFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not canonicalize path {}, error: {}", path.display(), source))]
    CanonicalizePath { path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not parse YAML configuration, error: {}", source))]
    ParseYamlConfig { source: serde_yaml::Error },

    #[snafu(display("Could not spawn external command {} {}, error: {}", command, args.join(" "), source))]
    SpawnExternalCommand { command: String, args: Vec<String>, source: std::io::Error },

    #[snafu(display("Could not wait for spawned process, error: {}", source))]
    WaitForSpawnedProcess { source: std::io::Error },
}
