//! Edit command implementation for editing operations.
//!
//! This command provides editing functionality with subcommands for
//! configuring axis properties and displaying edit state. It serves
//! as the main entry point for all editing-related operations.

use super::axis::AxisCommand;
use super::base::{ExitCommand, InfoCommand};
use super::show::ShowCommand;
use crate::{CliError, CliResult, Command, CommandResult};

/// Edit command handling "Axis", "Show", "Info", and "Exit"
///
/// This command provides a submenu for editing operations including
/// axis configuration, state display, and help functionality. It creates
/// context-aware subcommands that understand they are operating in edit mode.
#[derive(Debug)]
pub struct EditCommand;

impl Default for EditCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl EditCommand {
    /// Creates a new EditCommand instance
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::edit::EditCommand;
    /// let edit_cmd = EditCommand::new();
    /// ```
    pub fn new() -> Self {
        EditCommand
    }
}

impl Command for EditCommand {
    fn name(&self) -> &'static str {
        "edit"
    }

    fn description(&self) -> &'static str {
        "Edit operations: Axis, Show, Info, Exit"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["e"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - edit command takes no arguments when used as menu
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        Ok(CommandResult::Continue)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(AxisCommand::new("edit")),
            Box::new(ShowCommand::new("edit")),
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
