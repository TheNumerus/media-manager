use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Argument Parsing Error: {0}")]
    ArgsParse(String),
    #[error("Input Error: {0}, caused by {1}")]
    Input(String, #[source] std::io::Error),
    #[error(transparent)]
    Library(#[from] libmm::error::Error),
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
