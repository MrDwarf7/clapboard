#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("Invalid record mode: {0}")]
    InvalidRecordMode(String),

    #[error("Invalid configuration: {0}")]
    XdgDirectoryNotAvailable(String),

    #[error("Configuration parse error: {0}")]
    ConfigParse(#[from] config::ConfigError),
}
