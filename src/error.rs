#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to get user name from environment variable")]
    EnvUserNotFound,

    #[fail(display = "failed to get home from environment variable")]
    EnvHomeNotFound,

    #[fail(display = "no command provided")]
    NoCommandProvided,

    #[fail(display = "failed to read standard input")]
    StandardInput,

    #[fail(display = "IO error: {}", _0)]
    StdIo(#[cause] std::io::Error),

    #[fail(display = "Fs extra error: {}", _0)]
    FsExtra(fs_extra::error::Error),

    #[fail(display = "serde parser error: {}", _0)]
    SerdeYaml(#[cause] serde_yaml::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::StdIo(err)
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(err: fs_extra::error::Error) -> Error {
        Error::FsExtra(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::SerdeYaml(err)
    }
}
