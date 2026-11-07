#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("terminal error: {0}")]
    Terminal(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
