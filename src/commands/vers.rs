//! Version command implementation for displaying application version information.
//!
//! This command provides functionality to display the current version of the
//! application along with other relevant build information. It uses compile-time
//! environment variables to retrieve version information.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Vers command that shows version information
///
/// This command displays the current version of the application using
/// compile-time environment variables from Cargo. It provides a simple
/// way for users to check which version of the application they are running.
#[derive(Debug)]
pub struct VersCommand;

impl Default for VersCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl VersCommand {
    /// Creates a new VersCommand instance
    ///
    /// # Examples
    /// ```
    /// use simple_menu::commands::vers::VersCommand;
    /// let vers_cmd = VersCommand::new();
    /// ```
    pub fn new() -> Self {
        VersCommand
    }
}

impl Command for VersCommand {
    fn name(&self) -> &'static str {
        "vers"
    }

    fn description(&self) -> &'static str {
        "Show version information about the application"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["v"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - vers command takes no arguments
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }

        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");

        println!("{name} > version {version}");
        Ok(CommandResult::Continue)
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
