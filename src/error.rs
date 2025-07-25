pub use crate::prelude::Result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("Invalid recording mode: {0}")]
    InvalidRecordingMode(String),

    #[error("Invalid configuration: {0}")]
    XdgDirectoryNotAvailable(String),

    #[error("Configuration parse error: {0}")]
    ConfigParse(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid XDG directory type: {0}")]
    InvalidXdgDirType(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(#[from] wl_clipboard_rs::copy::Error),
}
