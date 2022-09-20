use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    ArgsParse(String),
    Input(String, std::io::Error),
    Library(libmm::error::Error),
    Config(String),
}

impl AppError {
    /// Convenience method for invalid input errors
    pub fn invalid_input(msg: impl AsRef<str>) -> Self {
        Self::Input(msg.as_ref().into(), std::io::ErrorKind::InvalidInput.into())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgsParse(msg) => f.write_fmt(format_args!("Argument Parsing Error: {msg}")),
            Self::Input(msg, err) => {
                f.write_fmt(format_args!("Input Error: {msg}, caused by {err}"))
            }
            Self::Library(err) => f.write_fmt(format_args!("{err}")),
            Self::Config(msg) => f.write_fmt(format_args!("Configuration Error: {msg}")),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Input(_, err) => Some(err),
            Self::Library(err) => Some(err),
            _ => None,
        }
    }
}

impl From<libmm::error::Error> for AppError {
    fn from(e: libmm::error::Error) -> Self {
        Self::Library(e)
    }
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ArgsParse(_), Self::ArgsParse(_)) => true,
            (Self::Input(_, e1), Self::Input(_, e2)) => e1.kind() == e2.kind(),
            (_, _) => false,
        }
    }
}
