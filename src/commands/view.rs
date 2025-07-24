//! View command implementation for viewing operations.
//!
//! This command provides viewing functionality with subcommands for
//! configuring axis properties and displaying view state. It serves
//! as the main entry point for all viewing-related operations.

use super::axis::AxisCommand;
use super::base::{ExitCommand, InfoCommand};
use super::show::ShowCommand;
use crate::{CliError, CliResult, Command, CommandResult};

/// View command handling "Axis", "Show", "Info", and "Exit"
///
/// This command provides a submenu for viewing operations including
/// axis configuration, state display, and help functionality. It creates
/// context-aware subcommands that understand they are operating in view mode.
#[derive(Debug)]
pub struct ViewCommand;

impl Default for ViewCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewCommand {
    /// Creates a new ViewCommand instance
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::view::ViewCommand;
    /// let view_cmd = ViewCommand::new();
    /// ```
    pub fn new() -> Self {
        ViewCommand
    }
}

impl Command for ViewCommand {
    fn name(&self) -> &'static str {
        "view"
    }

    fn description(&self) -> &'static str {
        "View operations: Axis, Show, Info, Exit"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["v"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - view command takes no arguments when used as menu
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
            Box::new(AxisCommand::new("view")),
            Box::new(ShowCommand::new("view")),
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
