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
    InvalidAppImage,
    InvalidSlug(String),
    CantUpdatePkg,

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

    #[from]
    Octocrab(octocrab::Error),

    #[from]
    Dialoguer(dialoguer::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        match self {
            Error::Io(e) => write!(fmt, "{e}"),
            Error::NotFound(name) => write!(fmt, "Application '{name}' not found"),
            Error::Json(e) => write!(fmt, "JSON error: {e}"),
            Error::Http(e) => write!(fmt, "HTTP error: {e}"),
            Error::EnvVar(e) => write!(fmt, "Environment variable error: {e}"),
            Error::InvalidPath => write!(fmt, "Invalid path provided"),
            Error::CantUpdatePkg => write!(fmt, "Can't update package"),
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
            Error::InvalidAppImage => {
                write!(fmt, "Invalid AppImage")
            }
            Error::InvalidSlug(slug) => write!(fmt, "Invalid repository slug {slug}"),
            Error::Octocrab(e) => write!(fmt, "Octocrab error: {e}"),
            Error::Dialoguer(e) => write!(fmt, "Dialoguer error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
