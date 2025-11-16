//! File command implementation for file operations.
//!
//! This command provides file operation functionality including loading,
//! saving, and version information. It serves as a submenu for all
//! file-related operations and demonstrates recursive command structure.

use super::base::{ExitCommand, InfoCommand};
use super::load::LoadCommand;
use super::save::SaveCommand;
use super::vers::VersCommand;
use crate::{CliError, CliResult, Command, CommandResult};

/// File command handling "Load", "Save", "Version", "Info", and "Exit"
///
/// This command provides a submenu for file operations including loading files,
/// saving files, displaying version information, and accessing help. It supports
/// recursive functionality by allowing nested file menus.
#[derive(Debug)]
pub struct FileCommand {
    #[allow(dead_code)]
    parent_context: Option<String>,
}

impl Default for FileCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCommand {
    /// Creates a new FileCommand with no parent context
    ///
    /// # Examples
    /// ```
    /// use sm_menu::commands::file::FileCommand;
    /// let file_cmd = FileCommand::new();
    /// ```
    pub fn new() -> Self {
        FileCommand {
            parent_context: None,
        }
    }

    /// Creates a new FileCommand with a parent context for recursive functionality
    ///
    /// # Arguments
    /// * `parent` - The parent context string
    ///
    /// # Examples
    /// ```
    /// use sm_menu::commands::file::FileCommand;
    /// let recursive_file_cmd = FileCommand::new_with_parent("file");
    /// ```
    pub fn new_with_parent(parent: &str) -> Self {
        FileCommand {
            parent_context: Some(parent.to_string()),
        }
    }
}

impl Command for FileCommand {
    fn name(&self) -> &'static str {
        "file"
    }

    fn description(&self) -> &'static str {
        "File operations: Load, Save, Version, Info, Exit"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["f"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Validate arguments - file command takes no arguments when used as menu
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
            Box::new(LoadCommand::new()),
            Box::new(SaveCommand::new()),
            Box::new(VersCommand::new()),
            // Recursive file command - creates a nested file menu
            Box::new(FileCommand::new_with_parent("file")),
            Box::new(InfoCommand::new(self.name())),
            Box::new(ExitCommand::new()),
        ]
    }
}
