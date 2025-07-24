//! Show command implementation for displaying current state.
//!
//! This command provides functionality to display the current state and
//! configuration within different contexts (edit, view). It gives users
//! insight into the current operational mode and settings.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Show command for displaying current state
///
/// This command displays the current state and configuration for different
/// contexts such as editing or viewing modes. It provides users with information
/// about the current operational state.
#[derive(Debug)]
pub struct ShowCommand {
    context: String,
}

impl ShowCommand {
    /// Creates a new ShowCommand with the specified context
    ///
    /// # Arguments
    /// * `context` - The context string (e.g., "edit", "view")
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::show::ShowCommand;
    /// let show_cmd = ShowCommand::new("edit");
    /// ```
    pub fn new(context: &str) -> Self {
        ShowCommand {
            context: context.to_string(),
        }
    }
}

impl Command for ShowCommand {
    fn name(&self) -> &'static str {
        "show"
    }

    fn description(&self) -> &'static str {
        match self.context.as_str() {
            "edit" => "Display current edit state and configuration",
            "view" => "Display current view state and configuration",
            _ => "Display current state",
        }
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["sh"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Show command takes no arguments
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        match self.context.as_str() {
            "edit" => {
                println!("Displaying current edit state...");
                println!("Edit mode: Active");
                println!("Current selection: None");
            }
            "view" => {
                println!("Displaying current view state...");
                println!("View mode: Active");
                println!("Current perspective: Default");
            }
            _ => {
                println!("Displaying current state...");
                println!("Status: Active");
            }
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
