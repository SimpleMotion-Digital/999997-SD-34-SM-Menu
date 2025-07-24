//! Core error handling module for the CLI application.
//!
//! This module defines the central error types and error handling utilities
//! used throughout the application. It provides a consistent error handling
//! strategy following Rust best practices.

use std::error::Error;
use std::fmt;
use std::io;

/// Result type for CLI operations
pub type CliResult<T> = Result<T, CliError>;

/// Central error type for all CLI operations
///
/// This enum represents all possible errors that can occur in the CLI application.
/// Each variant includes detailed information about the error context.
#[derive(Debug)]
pub enum CliError {
    /// Invalid command name provided
    InvalidCommand(String),
    /// Invalid input format or content
    InvalidInput(String),
    /// IO operation failed
    IoError(io::Error),
    /// Empty input provided when input was required
    EmptyInput,
    /// Too many arguments provided to command
    TooManyArguments { expected: usize, found: usize },
    /// Too few arguments provided to command
    TooFewArguments { expected: usize, found: usize },
    /// Command execution failed
    ExecutionError(String),
    /// Permission denied error
    PermissionDenied(String),
    /// File not found error
    FileNotFound(String),
    /// Invalid file format
    InvalidFileFormat(String),
    /// Operation interrupted by user
    Interrupted,
    /// Terminal operation failed
    TerminalError(String),
    /// Internal error (should not happen in normal operation)
    InternalError(String),
    /// Generic error with context
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InvalidCommand(cmd) => write!(f, "Invalid command: '{cmd}'"),
            CliError::InvalidInput(input) => write!(f, "Invalid input: {input}"),
            CliError::IoError(err) => write!(f, "IO error: {err}"),
            CliError::EmptyInput => write!(f, "Empty input provided"),
            CliError::TooManyArguments { expected, found } => {
                write!(f, "Too many arguments: expected {expected}, found {found}")
            }
            CliError::TooFewArguments { expected, found } => {
                write!(f, "Too few arguments: expected {expected}, found {found}")
            }
            CliError::ExecutionError(msg) => write!(f, "Command execution failed: {msg}"),
            CliError::PermissionDenied(resource) => write!(f, "Permission denied: {resource}"),
            CliError::FileNotFound(path) => write!(f, "File not found: {path}"),
            CliError::InvalidFileFormat(details) => write!(f, "Invalid file format: {details}"),
            CliError::Interrupted => write!(f, "Operation interrupted by user"),
            CliError::TerminalError(msg) => write!(f, "Terminal error: {msg}"),
            CliError::InternalError(msg) => {
                write!(f, "Internal error: {msg} (please report this bug)")
            }
            CliError::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => CliError::FileNotFound(err.to_string()),
            io::ErrorKind::PermissionDenied => CliError::PermissionDenied(err.to_string()),
            io::ErrorKind::Interrupted => CliError::Interrupted,
            _ => CliError::IoError(err),
        }
    }
}

/// Error factory for creating specific error types
///
/// This implementation provides convenient methods for creating common error types
/// while maintaining consistency across the application.
impl CliError {
    /// Create an invalid command error
    pub fn invalid_command(cmd: &str) -> Self {
        CliError::InvalidCommand(cmd.to_string())
    }

    /// Create an invalid input error
    pub fn invalid_input(input: &str) -> Self {
        CliError::InvalidInput(input.to_string())
    }

    /// Create an execution error
    pub fn execution_error(msg: &str) -> Self {
        CliError::ExecutionError(msg.to_string())
    }

    /// Create a file not found error
    pub fn file_not_found(path: &str) -> Self {
        CliError::FileNotFound(path.to_string())
    }

    /// Create a permission denied error
    pub fn permission_denied(resource: &str) -> Self {
        CliError::PermissionDenied(resource.to_string())
    }

    /// Create a terminal error
    pub fn terminal_error(msg: &str) -> Self {
        CliError::TerminalError(msg.to_string())
    }

    /// Create an internal error
    pub fn internal_error(msg: &str) -> Self {
        CliError::InternalError(msg.to_string())
    }

    /// Create a generic error
    pub fn other(msg: &str) -> Self {
        CliError::Other(msg.to_string())
    }
}

/// Error severity levels for display formatting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    /// Warning level - operation can continue
    Warning,
    /// Error level - operation failed but application continues
    Error,
    /// Critical level - severe error that may require application restart
    Critical,
}

impl CliError {
    /// Get the severity level of this error
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            CliError::InvalidCommand(_) | CliError::InvalidInput(_) | CliError::EmptyInput => {
                ErrorSeverity::Warning
            }
            CliError::TooManyArguments { .. } | CliError::TooFewArguments { .. } => {
                ErrorSeverity::Warning
            }
            CliError::ExecutionError(_)
            | CliError::FileNotFound(_)
            | CliError::PermissionDenied(_) => ErrorSeverity::Error,
            CliError::InvalidFileFormat(_) | CliError::Interrupted => ErrorSeverity::Error,
            CliError::IoError(_) | CliError::TerminalError(_) | CliError::Other(_) => {
                ErrorSeverity::Error
            }
            CliError::InternalError(_) => ErrorSeverity::Critical,
        }
    }

    /// Get the emoji icon for this error type
    pub fn icon(&self) -> &'static str {
        match self.severity() {
            ErrorSeverity::Warning => "⚠️",
            ErrorSeverity::Error => "❌",
            ErrorSeverity::Critical => "💥",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CliError::InvalidCommand("test".to_string());
        assert_eq!(format!("{error}"), "Invalid command: 'test'");

        let error = CliError::EmptyInput;
        assert_eq!(format!("{error}"), "Empty input provided");

        let error = CliError::TooManyArguments {
            expected: 2,
            found: 5,
        };
        assert_eq!(
            format!("{error}"),
            "Too many arguments: expected 2, found 5"
        );

        let error = CliError::TooFewArguments {
            expected: 2,
            found: 1,
        };
        assert_eq!(format!("{error}"), "Too few arguments: expected 2, found 1");

        let error = CliError::ExecutionError("command failed".to_string());
        assert_eq!(
            format!("{error}"),
            "Command execution failed: command failed"
        );
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(
            CliError::InvalidCommand("test".to_string()).severity(),
            ErrorSeverity::Warning
        );
        assert_eq!(
            CliError::ExecutionError("test".to_string()).severity(),
            ErrorSeverity::Error
        );
        assert_eq!(
            CliError::InternalError("test".to_string()).severity(),
            ErrorSeverity::Critical
        );
    }

    #[test]
    fn test_error_icons() {
        assert_eq!(CliError::InvalidCommand("test".to_string()).icon(), "⚠️");
        assert_eq!(CliError::ExecutionError("test".to_string()).icon(), "❌");
        assert_eq!(CliError::InternalError("test".to_string()).icon(), "💥");
    }
}
