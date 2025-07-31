use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    InvalidPath,
    NotFound(String),

    #[from]
    Io(std::io::Error),

    #[from]
    Json(serde_json::Error),

    #[from]
    Http(reqwest::Error),

    #[from]
    EnvVar(std::env::VarError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        match self {
            Error::Io(e) => match e.kind() {
                std::io::ErrorKind::NotFound => write!(fmt, "File or directory not found"),
                _ => write!(fmt, "IO error: {e}"),
            },
            Error::NotFound(name) => write!(fmt, "Application '{name}' not found"),
            Error::Json(e) => write!(fmt, "JSON error: {e}"),
            Error::Http(e) => write!(fmt, "HTTP error: {e}"),
            Error::EnvVar(e) => write!(fmt, "Environment variable error: {e}"),
            Error::InvalidPath => write!(fmt, "Invalid path provided"),
        }
    }
}

impl std::error::Error for Error {}
