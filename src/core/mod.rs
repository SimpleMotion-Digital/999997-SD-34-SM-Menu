//! Core modules for the CLI application.
//!
//! This module contains the foundational components that provide the basic
//! functionality for the CLI application including error handling, command
//! abstractions, and context management.

pub mod command;
pub mod context;
pub mod error;

// Re-export commonly used types
pub use command::{ArgumentValidator, Command, CommandCategory, CommandResult};
pub use context::{CliContext, CliPreferences};
pub use error::{CliError, CliResult};
