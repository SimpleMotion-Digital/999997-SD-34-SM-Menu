//! Quit command implementation for program termination.
//!
//! This command provides functionality to gracefully exit the program.
//! It ensures proper cleanup and displays a goodbye message before
//! terminating the application.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Quit command that exits the program
///
/// This command provides a graceful way to exit the program with
/// a goodbye message. It ensures proper cleanup and termination
/// of the application when the user is ready to exit.
#[derive(Debug)]
pub struct QuitCommand;

impl Default for QuitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl QuitCommand {
    /// Creates a new QuitCommand instance
    ///
    /// # Examples
    /// ```
    /// use sm_menu::commands::quit::QuitCommand;
    /// let quit_cmd = QuitCommand::new();
    /// ```
    pub fn new() -> Self {
        QuitCommand
    }
}

impl Command for QuitCommand {
    fn name(&self) -> &'static str {
        "quit"
    }

    fn description(&self) -> &'static str {
        "Quit the program and return to the shell"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["q"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - quit command takes no arguments
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        println!("Goodbye!");
        Ok(CommandResult::Quit)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
