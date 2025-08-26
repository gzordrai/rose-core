use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bollard::errors::Error as DockerError;
use config::ConfigError;
use serde::Serialize;
use std::io::Error as IoError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RoseError>;

/// Represents all possible errors that can occur in the application.
///
/// This enum unifies errors from IO operations, configuration loading,
/// and Docker API interactions for both API responses and internal handling.
#[derive(Debug, Error)]
pub enum RoseError {
    /// IO error, wraps a standard IO error.
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    /// Configuration error, wraps a config crate error.
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),

    /// Docker error, wraps a Bollard Docker API error.
    #[error("Docker error: {0}")]
    Docker(#[from] DockerError),
}

/// Represents the structure of error responses returned by the API.
#[derive(Serialize)]
struct ErrorBody {
    /// The error message describing the problem.
    error: String,
}

/// Converts a `RoseError` into an HTTP response for Axum.
///
/// This enables using `?` in handlers without manual error mapping
impl IntoResponse for RoseError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            RoseError::Docker(_) => (StatusCode::SERVICE_UNAVAILABLE, "Docker unavailable"),
            RoseError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Config error"),
            RoseError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
        };

        (status, Json(ErrorBody { error: msg.into() })).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use std::io::Error;

    #[test]
    fn test_io_error_response() {
        let io_err = RoseError::Io(Error::other("io error"));
        let response = io_err.into_response();
        let status = response.status();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_config_error_response() {
        let config_err = RoseError::Config(ConfigError::Frozen);
        let response = config_err.into_response();
        let status = response.status();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_docker_error_response() {
        let docker_err = RoseError::Docker(DockerError::IOError {
            err: Error::other("docker fail"),
        });
        let response = docker_err.into_response();
        let status = response.status();

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }
}
