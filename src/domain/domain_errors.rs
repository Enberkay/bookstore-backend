use thiserror::Error;
use anyhow::Result;

/// Common error type across the domain layer.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Business rule violated: {0}")]
    RuleViolation(String),

    #[error("Operation not permitted: {0}")]
    PermissionDenied(String),

    #[error("Unexpected domain error: {0}")]
    Unexpected(String),
}

impl DomainError {
    pub fn validation<T: Into<String>>(msg: T) -> Self {
        Self::ValidationError(msg.into())
    }

    pub fn not_found<T: Into<String>>(entity: T) -> Self {
        Self::NotFound(entity.into())
    }

    pub fn rule_violation<T: Into<String>>(msg: T) -> Self {
        Self::RuleViolation(msg.into())
    }

    pub fn unexpected<T: Into<String>>(msg: T) -> Self {
        Self::Unexpected(msg.into())
    }
}

/// Shortcut alias for domain-level Result
pub type DomainResult<T> = Result<T, DomainError>;
