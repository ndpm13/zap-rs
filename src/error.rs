use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    InvalidPath,
    NotFound(String),
    Download {
        url: String,
        source: reqwest::Error,
    },

    #[from]
    Io(std::io::Error),

    #[from]
    Json(serde_json::Error),

    #[from]
    Http(reqwest::Error),

    #[from]
    EnvVar(std::env::VarError),

    #[from]
    IndicatifTemplate(indicatif::style::TemplateError),
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
            Error::IndicatifTemplate(e) => write!(fmt, "Progress bar template error: {e}"),
            Error::Download { url, source } => {
                if source.is_timeout() {
                    write!(fmt, "Download timed out from: {url}")
                } else if source.is_connect() {
                    write!(fmt, "Failed to connect to: {url}")
                } else if let Some(status) = source.status() {
                    match status.as_u16() {
                        404 => write!(fmt, "AppImage not found at: {url}"),
                        403 => write!(fmt, "Access denied at: {url}"),
                        500..=599 => write!(fmt, "Server error when downloading from: {url}"),
                        _ => write!(fmt, "HTTP {status} error downloading from: {url}"),
                    }
                } else {
                    write!(fmt, "Failed to download from {url}: {source}")
                }
            }
        }
    }
}

impl std::error::Error for Error {}
