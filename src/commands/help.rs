//! Help command implementation for displaying command information.
//!
//! This command provides comprehensive help functionality including
//! general help for all commands and specific help for individual
//! commands. It formats output with visual enhancements like bold
//! text for improved readability.

use super::base::{ExitCommand, InfoCommand};
use crate::{CliError, CliResult, Command, CommandResult};

/// Format a command name with bold alias character
///
/// This function formats command names with ANSI escape codes to make
/// the alias character bold for better visual distinction.
///
/// # Arguments
/// * `name` - The command name to format
/// * `alias` - Optional alias character to make bold
///
/// # Returns
/// A formatted string with the alias character in bold
fn format_command_with_alias(name: &str, alias: Option<&str>) -> String {
    if let Some(alias_char) = alias {
        if let Some(first_char) = name.chars().next() {
            if alias_char.starts_with(first_char) {
                // Bold the first character if it matches the alias
                format!("\x1b[1m{}\x1b[0m{}", first_char, &name[1..])
            } else {
                name.to_string()
            }
        } else {
            name.to_string()
        }
    } else {
        name.to_string()
    }
}

/// Help command displaying available commands and their descriptions
///
/// This command provides comprehensive help functionality including
/// general help that lists all available commands and specific help
/// for individual commands. It supports both general help display
/// and command-specific help information.
#[derive(Debug)]
pub struct HelpCommand;

impl Default for HelpCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpCommand {
    /// Creates a new HelpCommand instance
    ///
    /// # Examples
    /// ```
    /// use sm_menu::commands::help::HelpCommand;
    /// let help_cmd = HelpCommand::new();
    /// ```
    pub fn new() -> Self {
        HelpCommand
    }
}

impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Help information for the available commands"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["h"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Help command can take 0 or 1 arguments (optional command name for specific help)
        if args.len() > 1 {
            return Err(CliError::TooManyArguments {
                expected: 1,
                found: args.len(),
            });
        }

        if args.is_empty() {
            // Show general help
            println!("simple-menu Help");
            println!("===========");
            println!("Available commands:");
            println!(
                "  {} - File operations: Load, Save, Version, Info, Exit",
                format_command_with_alias("file", Some("f"))
            );
            println!(
                "  {} - Edit operations: Axis, Show, Info, Exit",
                format_command_with_alias("edit", Some("e"))
            );
            println!(
                "  {} - View operations: Axis, Show, Info, Exit",
                format_command_with_alias("view", Some("v"))
            );
            println!(
                "  {} - Display help information for available commands",
                format_command_with_alias("help", Some("h"))
            );
            println!(
                "  {} - Exit the program",
                format_command_with_alias("quit", Some("q"))
            );
            println!();
            println!("Type a command name to enter its submenu or see its options.");
            println!("Use 'help <command>' for specific command help.");
        } else {
            // Show specific command help
            let command_name = &args[0];
            match command_name.as_str() {
                "file" | "f" => {
                    println!("File Command Help");
                    println!("=================");
                    println!("The file command provides file operation functionality.");
                    println!("Subcommands:");
                    println!("  load <filename> - Load a file");
                    println!("  save [filename] - Save a file (default: untitled.txt)");
                    println!("  vers - Show version information");
                    println!("  info - Show file menu information");
                    println!("  exit - Return to main menu");
                }
                "edit" | "e" => {
                    println!("Edit Command Help");
                    println!("=================");
                    println!("The edit command provides editing functionality.");
                    println!("Subcommands:");
                    println!("  axis [name] - Configure axis properties");
                    println!("  show - Display current edit state");
                    println!("  info - Show edit menu information");
                    println!("  exit - Return to main menu");
                }
                "view" | "v" => {
                    println!("View Command Help");
                    println!("=================");
                    println!("The view command provides viewing functionality.");
                    println!("Subcommands:");
                    println!("  axis [name] - Configure axis properties for viewing");
                    println!("  show - Display current view state");
                    println!("  info - Show view menu information");
                    println!("  exit - Return to main menu");
                }
                "help" | "h" => {
                    println!("Help Command Help");
                    println!("=================");
                    println!("The help command displays information about available commands.");
                    println!("Usage:");
                    println!("  help        - Show general help");
                    println!("  help <cmd>  - Show specific command help");
                }
                "quit" | "q" => {
                    println!("Quit Command Help");
                    println!("=================");
                    println!("The quit command exits the program.");
                    println!("Usage: quit (no arguments)");
                }
                _ => {
                    return Err(CliError::invalid_input(&format!(
                        "No help available for command: {command_name}"
                    )));
                }
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
