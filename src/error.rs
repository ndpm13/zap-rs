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
