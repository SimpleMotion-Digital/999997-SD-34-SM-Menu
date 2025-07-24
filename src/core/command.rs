//! Command trait and related types for the CLI application.
//!
//! This module defines the core command abstraction that all CLI commands must implement.
//! It provides a consistent interface for command execution, help generation, and
//! subcommand management.

use crate::core::error::CliResult;

/// Result of command execution
///
/// This enum represents the different outcomes of executing a command,
/// allowing the CLI framework to handle navigation and control flow.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    /// Command executed successfully with an optional message
    Success(String),
    /// Return to parent menu
    GoUp,
    /// Exit the program
    Quit,
    /// Continue in current menu context
    Continue,
}

impl CommandResult {
    /// Create a success result with a message
    pub fn success(msg: impl Into<String>) -> Self {
        CommandResult::Success(msg.into())
    }

    /// Create a success result with no message
    pub fn success_silent() -> Self {
        CommandResult::Success(String::new())
    }
}

/// Core trait that all CLI commands must implement
///
/// This trait defines the interface for all commands in the CLI application.
/// It provides methods for command identification, execution, and help generation.
pub trait Command: std::fmt::Debug {
    /// Get the command name (used for command matching)
    fn name(&self) -> &'static str;

    /// Get the command description for help text
    fn description(&self) -> &'static str;

    /// Execute the command with given arguments
    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult>;

    /// Get available subcommands (if any)
    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        Vec::new()
    }

    /// Get command aliases (alternative names for the command)
    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Get detailed help text for the command
    fn help(&self) -> String {
        let aliases = self.aliases();
        if aliases.is_empty() {
            format!("{} - {}", self.name(), self.description())
        } else {
            format!(
                "{} ({}) - {}",
                self.name(),
                aliases.join(", "),
                self.description()
            )
        }
    }

    /// Check if this command has subcommands
    fn has_subcommands(&self) -> bool {
        !self.subcommands().is_empty()
    }

    /// Check if the given name matches this command (name or alias)
    fn matches(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.name().to_lowercase() == name_lower
            || self
                .aliases()
                .iter()
                .any(|alias| alias.to_lowercase() == name_lower)
    }

    /// Get usage information for the command
    fn usage(&self) -> String {
        format!("{} [OPTIONS]", self.name())
    }

    /// Get command category for help organization
    fn category(&self) -> CommandCategory {
        CommandCategory::General
    }
}

/// Command categories for organizing help output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    /// General commands
    General,
    /// File operations
    File,
    /// Edit operations
    Edit,
    /// View operations
    View,
    /// System commands
    System,
}

impl CommandCategory {
    /// Get the display name for this category
    pub fn display_name(&self) -> &'static str {
        match self {
            CommandCategory::General => "General",
            CommandCategory::File => "File Operations",
            CommandCategory::Edit => "Edit Operations",
            CommandCategory::View => "View Operations",
            CommandCategory::System => "System",
        }
    }
}

/// Command registry for managing available commands
///
/// This struct provides a centralized way to manage and access commands,
/// supporting features like command discovery and help generation.
#[derive(Debug)]
pub struct CommandRegistry {
    commands: Vec<Box<dyn Command>>,
}

impl CommandRegistry {
    /// Create a new command registry
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Register a command with the registry
    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// Find a command by name or alias
    pub fn find_command(&self, name: &str) -> Option<&dyn Command> {
        self.commands
            .iter()
            .find(|cmd| cmd.matches(name))
            .map(|cmd| cmd.as_ref())
    }

    /// Get all registered commands
    pub fn commands(&self) -> &[Box<dyn Command>] {
        &self.commands
    }

    /// Get commands organized by category
    pub fn commands_by_category(
        &self,
    ) -> std::collections::HashMap<CommandCategory, Vec<&dyn Command>> {
        let mut categorized = std::collections::HashMap::new();

        for command in &self.commands {
            categorized
                .entry(command.category())
                .or_insert_with(Vec::new)
                .push(command.as_ref());
        }

        categorized
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for commands that can validate their arguments
pub trait ArgumentValidator {
    /// Validate the number of arguments
    fn validate_arg_count(&self, args: &[String], expected: usize) -> CliResult<()> {
        if args.len() != expected {
            return Err(crate::core::error::CliError::TooManyArguments {
                expected,
                found: args.len(),
            });
        }
        Ok(())
    }

    /// Validate the number of arguments is within a range
    fn validate_arg_range(&self, args: &[String], min: usize, max: usize) -> CliResult<()> {
        if args.len() < min {
            return Err(crate::core::error::CliError::TooFewArguments {
                expected: min,
                found: args.len(),
            });
        }
        if args.len() > max {
            return Err(crate::core::error::CliError::TooManyArguments {
                expected: max,
                found: args.len(),
            });
        }
        Ok(())
    }

    /// Validate that an argument is not empty
    fn validate_not_empty(&self, arg: &str, arg_name: &str) -> CliResult<()> {
        if arg.trim().is_empty() {
            return Err(crate::core::error::CliError::invalid_input(&format!(
                "{arg_name} cannot be empty"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCommand {
        name: &'static str,
        description: &'static str,
    }

    impl Command for TestCommand {
        fn name(&self) -> &'static str {
            self.name
        }

        fn description(&self) -> &'static str {
            self.description
        }

        fn execute(&mut self, _args: &[String]) -> CliResult<CommandResult> {
            Ok(CommandResult::Success("Test executed".to_string()))
        }

        fn aliases(&self) -> Vec<&'static str> {
            vec!["t"]
        }
    }

    #[test]
    fn test_command_matches() {
        let cmd = TestCommand {
            name: "test",
            description: "Test command",
        };

        assert!(cmd.matches("test"));
        assert!(cmd.matches("t"));
        assert!(!cmd.matches("other"));
    }

    #[test]
    fn test_command_matches_case_insensitive() {
        let cmd = TestCommand {
            name: "test",
            description: "Test command",
        };

        // Test case insensitive matching for command name
        assert!(cmd.matches("test"));
        assert!(cmd.matches("Test"));
        assert!(cmd.matches("TEST"));
        assert!(cmd.matches("tEsT"));

        // Test case insensitive matching for aliases
        assert!(cmd.matches("t"));
        assert!(cmd.matches("T"));

        // Test that non-matches still fail
        assert!(!cmd.matches("other"));
        assert!(!cmd.matches("OTHER"));
    }

    #[test]
    fn test_command_help() {
        let cmd = TestCommand {
            name: "test",
            description: "Test command",
        };

        assert_eq!(cmd.help(), "test (t) - Test command");
    }

    #[test]
    fn test_command_result() {
        let result = CommandResult::success("Test message");
        assert_eq!(result, CommandResult::Success("Test message".to_string()));

        let result = CommandResult::success_silent();
        assert_eq!(result, CommandResult::Success(String::new()));
    }

    #[test]
    fn test_command_registry() {
        let mut registry = CommandRegistry::new();
        let cmd = Box::new(TestCommand {
            name: "test",
            description: "Test command",
        });

        registry.register(cmd);
        assert!(registry.find_command("test").is_some());
        assert!(registry.find_command("t").is_some());
        assert!(registry.find_command("other").is_none());
    }

    #[test]
    fn test_command_registry_case_insensitive() {
        let mut registry = CommandRegistry::new();
        let cmd = Box::new(TestCommand {
            name: "test",
            description: "Test command",
        });

        registry.register(cmd);

        // Test case insensitive matching for command name
        assert!(registry.find_command("test").is_some());
        assert!(registry.find_command("Test").is_some());
        assert!(registry.find_command("TEST").is_some());
        assert!(registry.find_command("tEsT").is_some());

        // Test case insensitive matching for aliases
        assert!(registry.find_command("t").is_some());
        assert!(registry.find_command("T").is_some());

        // Test that non-matches still fail
        assert!(registry.find_command("other").is_none());
        assert!(registry.find_command("OTHER").is_none());
    }
}
