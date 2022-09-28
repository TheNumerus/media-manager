use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApiError {
    ApiKey,
    InvalidFormat,
    Transport(ureq::Transport),
    Unknown(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKey => f.write_str("Invalid API key"),
            Self::InvalidFormat => f.write_str("Invalid format of API response"),
            Self::Transport(err) => f.write_fmt(format_args!("Transport Error: {err}")),
            Self::Unknown(msg) => f.write_str(msg),
        }
    }
}

impl From<ureq::Transport> for ApiError {
    fn from(t: ureq::Transport) -> Self {
        Self::Transport(t)
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Transport(t) => Some(t),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum DbError {
    Sqlite(rusqlite::Error),
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlite(e) => f.write_fmt(format_args!("SQLite error: {e}")),
        }
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbError::Sqlite(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum MediaError {
    Matroska(matroska::MatroskaError),
    NoVideoTrack,
    IncompleteMetadata,
}

impl Display for MediaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaError::Matroska(e) => f.write_fmt(format_args!("Matroska error: {e}")),
            MediaError::NoVideoTrack => f.write_str("No video track was found"),
            MediaError::IncompleteMetadata => f.write_str("Cannot real all needed metadata"),
        }
    }
}

impl From<matroska::MatroskaError> for MediaError {
    fn from(e: matroska::MatroskaError) -> Self {
        Self::Matroska(e)
    }
}

impl std::error::Error for MediaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MediaError::Matroska(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Db(DbError),
    Media(MediaError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(e) => f.write_fmt(format_args!("{e}")),
            Self::Db(e) => f.write_fmt(format_args!("{e}")),
            Self::Media(e) => f.write_fmt(format_args!("{e}")),
        }
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self::Api(e)
    }
}

impl From<DbError> for Error {
    fn from(e: DbError) -> Self {
        Self::Db(e)
    }
}

impl From<MediaError> for Error {
    fn from(e: MediaError) -> Self {
        Self::Media(e)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(DbError::Sqlite(e))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Api(e) => Some(e),
            Error::Db(e) => Some(e),
            Error::Media(e) => Some(e),
        }
    }
}
