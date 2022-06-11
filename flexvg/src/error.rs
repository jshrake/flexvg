use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlexError {
    #[error("Taffy error")]
    Taffy(#[from] taffy::Error),
    #[error("File IO error")]
    IO(#[from] std::io::Error),
    #[error("unknown error")]
    Unknown,
}
