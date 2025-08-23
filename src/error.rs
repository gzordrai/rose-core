use bollard::errors::Error as DockerError;
use config::ConfigError;
use std::io::Error as IoError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RoseError>;

#[derive(Debug, Error)]
pub enum RoseError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("Config error: {0}")]
    Config(#[from] ConfigError),

    #[error("Docker error: {0}")]
    Docker(#[from] DockerError),
}
