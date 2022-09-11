use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid API key")]
    ApiKey,
    #[error("Invalid format of API response")]
    InvalidFormat,
    #[error("Transport Error: {0}")]
    Transport(#[from] ureq::Transport),
    #[error("{0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    Db(#[from] DbError),
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(DbError::Sqlite(e))
    }
}
