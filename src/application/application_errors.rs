use thiserror::Error;
use anyhow::Result;

/// Common error type across the application (service/API) layer.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal application error: {0}")]
    Internal(String),
}

impl ApplicationError {
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn unauthorized<T: Into<String>>(msg: T) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn forbidden<T: Into<String>>(msg: T) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn conflict<T: Into<String>>(msg: T) -> Self {
        Self::Conflict(msg.into())
    }

    pub fn internal<T: Into<String>>(msg: T) -> Self {
        Self::Internal(msg.into())
    }
}

/// Shortcut alias for application-level Result
pub type ApplicationResult<T> = Result<T, ApplicationError>;
