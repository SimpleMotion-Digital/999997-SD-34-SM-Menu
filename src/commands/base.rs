//! Base command implementations that are commonly used across the CLI.
//!
//! This module provides foundational command implementations that can be
//! reused by various menus throughout the application.

use crate::{CliError, CliResult, Command, CommandResult};

/// Base info command that displays information about the current menu
///
/// This command can be used in any menu to show contextual information
/// about the available commands and menu functionality.
#[derive(Debug)]
pub struct InfoCommand {
    pub command: &'static str,
}

impl InfoCommand {
    pub fn new(command: &'static str) -> Self {
        InfoCommand { command }
    }
}

impl Command for InfoCommand {
    fn name(&self) -> &'static str {
        "info"
    }

    fn description(&self) -> &'static str {
        "Display information about the current menu"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["i"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - info command takes no arguments
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        println!("{} menu information:", self.command);
        println!("Available commands in this menu:");
        println!("  Type any command name to execute it");
        println!("  Use 'exit' (or 'e') to return to parent menu");
        Ok(CommandResult::Success(String::new()))
    }
}

/// Base exit command that returns to the parent menu
///
/// This command provides a consistent way to navigate back to the parent
/// menu from any submenu. It uses the `CommandResult::GoUp` to signal
/// the navigation system to pop the current menu from the stack.
#[derive(Debug)]
pub struct ExitCommand;

impl ExitCommand {
    pub fn new() -> Self {
        ExitCommand
    }
}

impl Default for ExitCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit to the parent menu"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["e"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - exit command takes no arguments
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        Ok(CommandResult::GoUp)
    }
}
