//! Load command implementation for loading files from the filesystem.
//!
//! This command provides functionality to load files from the filesystem
//! with comprehensive validation and error handling. It ensures safe file
//! operations and prevents directory traversal attacks.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Load command for loading files from the filesystem
///
/// This command handles file loading operations with security validation
/// to prevent directory traversal attacks and ensure safe file access.
/// It requires exactly one argument (the filename) and provides meaningful
/// error messages for various failure conditions.
#[derive(Debug)]
pub struct LoadCommand;

impl Default for LoadCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadCommand {
    /// Creates a new LoadCommand instance
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::load::LoadCommand;
    /// let load_cmd = LoadCommand::new();
    /// ```
    pub fn new() -> Self {
        LoadCommand
    }
}

impl Command for LoadCommand {
    fn name(&self) -> &'static str {
        "load"
    }

    fn description(&self) -> &'static str {
        "Load a file from the filesystem"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["l"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Load command expects exactly one argument (filename)
        if args.is_empty() {
            return Err(CliError::TooFewArguments {
                expected: 1,
                found: 0,
            });
        }

        if args.len() > 1 {
            return Err(CliError::TooManyArguments {
                expected: 1,
                found: args.len(),
            });
        }

        let filename = &args[0];

        // Validate filename
        if filename.trim().is_empty() {
            return Err(CliError::invalid_input("Filename cannot be empty"));
        }

        // Simulate file loading with error handling
        if filename.contains("..") {
            return Err(CliError::invalid_input(
                "Invalid filename: path traversal not allowed",
            ));
        }

        println!("Loading file: {filename}");

        // Simulate file not found error for demonstration
        if filename.ends_with(".missing") {
            return Err(CliError::file_not_found(filename));
        }

        Ok(CommandResult::Continue)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
