//! Axis command implementation for configuring axis properties.
//!
//! This command provides axis configuration functionality that can be used
//! in different contexts (edit, view) to configure axis properties for
//! the respective environment.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Axis command for configuring axis properties
///
/// This command allows users to configure axis properties within different
/// contexts such as editing or viewing modes. It supports optional axis name
/// parameters with validation for proper naming conventions.
#[derive(Debug)]
pub struct AxisCommand {
    context: String,
}

impl AxisCommand {
    /// Creates a new AxisCommand with the specified context
    ///
    /// # Arguments
    /// * `context` - The context string (e.g., "edit", "view")
    ///
    /// # Examples
    /// ```
    /// use sm_menu::commands::axis::AxisCommand;
    /// let axis_cmd = AxisCommand::new("edit");
    /// ```
    pub fn new(context: &str) -> Self {
        AxisCommand {
            context: context.to_string(),
        }
    }
}

impl Command for AxisCommand {
    fn name(&self) -> &'static str {
        "axis"
    }

    fn description(&self) -> &'static str {
        match self.context.as_str() {
            "edit" => "Configure axis properties for the editing environment",
            "view" => "Configure axis properties for the viewing environment",
            _ => "Configure axis properties",
        }
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["a"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Axis command can take 0 or 1 arguments (optional axis name)
        if args.len() > 1 {
            return Err(CliError::TooManyArguments {
                expected: 1,
                found: args.len(),
            });
        }

        let axis_name = if args.is_empty() {
            "default".to_string()
        } else {
            args[0].clone()
        };

        // Validate axis name
        if axis_name.trim().is_empty() {
            return Err(CliError::invalid_input("Axis name cannot be empty"));
        }

        // Validate axis name format
        if !axis_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(CliError::invalid_input(
                "Axis name can only contain alphanumeric characters, underscores, and hyphens",
            ));
        }

        match self.context.as_str() {
            "edit" => println!("Configuring axis properties for editing: {axis_name}"),
            "view" => println!("Configuring axis properties for viewing: {axis_name}"),
            _ => println!("Configuring axis properties: {axis_name}"),
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
