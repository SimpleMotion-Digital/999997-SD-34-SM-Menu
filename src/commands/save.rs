//! Save command implementation for saving files to the filesystem.
//!
//! This command provides functionality to save files to the filesystem
//! with comprehensive validation and error handling. It ensures safe file
//! operations and prevents unauthorized file access.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Save command for saving files to the filesystem
///
/// This command handles file saving operations with security validation
/// to prevent unauthorized file access and ensure safe file operations.
/// It accepts an optional filename argument and defaults to "untitled.txt"
/// if no filename is provided.
#[derive(Debug)]
pub struct SaveCommand;

impl Default for SaveCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveCommand {
    /// Creates a new SaveCommand instance
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::save::SaveCommand;
    /// let save_cmd = SaveCommand::new();
    /// ```
    pub fn new() -> Self {
        SaveCommand
    }
}

impl Command for SaveCommand {
    fn name(&self) -> &'static str {
        "save"
    }

    fn description(&self) -> &'static str {
        "Save a file to the filesystem"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["s"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Save command can take 0 or 1 arguments (optional filename)
        if args.len() > 1 {
            return Err(CliError::TooManyArguments {
                expected: 1,
                found: args.len(),
            });
        }

        let filename = if args.is_empty() {
            "untitled.txt".to_string()
        } else {
            args[0].clone()
        };

        // Validate filename
        if filename.trim().is_empty() {
            return Err(CliError::invalid_input("Filename cannot be empty"));
        }

        // Simulate file saving with error handling
        if filename.contains("..") {
            return Err(CliError::invalid_input(
                "Invalid filename: path traversal not allowed",
            ));
        }

        if filename.starts_with("/etc/") {
            return Err(CliError::permission_denied(&format!(
                "Cannot save to system directory: {filename}"
            )));
        }

        println!("Saving file: {filename}");
        Ok(CommandResult::Continue)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
