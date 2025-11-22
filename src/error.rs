//! Error types for Bunsenite
//!
//! This module provides comprehensive error handling for all Bunsenite operations.
//! Errors are designed to be informative and actionable for end users.

use std::fmt;

/// Result type alias for Bunsenite operations
pub type Result<T> = std::result::Result<T, Error>;

/// Bunsenite error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Nickel parsing error
    #[error("Failed to parse Nickel file '{file}': {message}")]
    ParseError {
        /// Name of the file that failed to parse
        file: String,
        /// Error message from the parser
        message: String,
    },

    /// Nickel evaluation error
    #[error("Failed to evaluate Nickel program '{file}': {message}")]
    EvaluationError {
        /// Name of the file that failed to evaluate
        file: String,
        /// Error message from the evaluator
        message: String,
    },

    /// Serialization error (converting Nickel values to JSON)
    #[error("Failed to serialize result: {0}")]
    SerializationError(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Internal error (should not happen in normal operation)
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Create a new parse error
    pub fn parse_error(file: impl Into<String>, message: impl Into<String>) -> Self {
        Error::ParseError {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a new evaluation error
    pub fn evaluation_error(file: impl Into<String>, message: impl Into<String>) -> Self {
        Error::EvaluationError {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a new serialization error
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Error::SerializationError(message.into())
    }

    /// Create a new invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Error::InvalidInput(message.into())
    }

    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Error::Internal(message.into())
    }

    /// Check if this error is recoverable
    ///
    /// Recoverable errors are those that the user can fix by changing input.
    /// Non-recoverable errors indicate bugs or system issues.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::ParseError { .. } | Error::InvalidInput(_) | Error::EvaluationError { .. }
        )
    }

    /// Get suggested fix for this error
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            Error::ParseError { .. } => Some("Check your Nickel syntax. Run 'nickel check' for detailed diagnostics."),
            Error::EvaluationError { .. } => Some("Ensure all variables are defined and types match."),
            Error::InvalidInput(_) => Some("Check the input format and try again."),
            Error::SerializationError(_) => Some("Ensure the Nickel program produces valid JSON-serializable values."),
            Error::IoError(_) => Some("Check file permissions and path."),
            Error::Internal(_) => Some("This is a bug. Please report it at: https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::parse_error("test.ncl", "syntax error");
        assert!(err.is_recoverable());
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn test_error_display() {
        let err = Error::parse_error("config.ncl", "unexpected token");
        let msg = format!("{}", err);
        assert!(msg.contains("config.ncl"));
        assert!(msg.contains("unexpected token"));
    }

    #[test]
    fn test_recoverable_errors() {
        assert!(Error::parse_error("test", "msg").is_recoverable());
        assert!(Error::invalid_input("msg").is_recoverable());
        assert!(!Error::internal("msg").is_recoverable());
    }
}
