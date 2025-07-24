//! Display and formatting utilities for the CLI interface.
//!
//! This module provides consistent formatting and display functionality
//! for the CLI application, including error display, help formatting,
//! and terminal management.

use crate::core::error::ErrorSeverity;
use crate::core::{CliError, Command};
use std::io::{self, Write};

/// Display manager for handling CLI output formatting
pub struct DisplayManager {
    /// Whether to use colored output
    colored: bool,
    /// Whether to use unicode symbols
    unicode: bool,
}

impl DisplayManager {
    /// Create a new display manager with default settings
    pub fn new() -> Self {
        Self {
            colored: true,
            unicode: true,
        }
    }

    /// Create a display manager with specific settings
    pub fn with_options(colored: bool, unicode: bool) -> Self {
        Self { colored, unicode }
    }

    /// Display an error with appropriate formatting
    pub fn display_error(&self, error: &CliError, command_stack: &[Box<dyn Command>]) {
        let icon = if self.unicode {
            error.icon()
        } else {
            match error.severity() {
                ErrorSeverity::Warning => "!",
                ErrorSeverity::Error => "X",
                ErrorSeverity::Critical => "!!",
            }
        };

        let color = if self.colored {
            match error.severity() {
                ErrorSeverity::Warning => "\x1b[1;33m",  // Yellow
                ErrorSeverity::Error => "\x1b[1;31m",    // Red
                ErrorSeverity::Critical => "\x1b[1;35m", // Magenta
            }
        } else {
            ""
        };

        let reset = if self.colored { "\x1b[0m" } else { "" };

        eprintln!("{color}{icon} {error}{reset}");

        // Show available commands for invalid command errors
        if matches!(error, CliError::InvalidCommand(_)) {
            self.display_available_commands(command_stack);
        }
    }

    /// Display available commands in a formatted list
    pub fn display_available_commands(&self, command_stack: &[Box<dyn Command>]) {
        if let Some(current_command) = command_stack.last() {
            let subcommands = current_command.subcommands();
            if !subcommands.is_empty() {
                for cmd in subcommands {
                    // Skip the info command - it's a secret command
                    if cmd.name() == "info" {
                        continue;
                    }

                    let aliases = cmd.aliases();
                    let alias_text = if aliases.is_empty() {
                        String::new()
                    } else {
                        let uppercase_aliases: Vec<String> =
                            aliases.iter().map(|a| a.to_uppercase()).collect();
                        format!(" ({})", uppercase_aliases.join(", "))
                    };

                    let formatted_name = if self.colored {
                        format!("\x1b[1;36m{}\x1b[0m", self.format_command_name(cmd.name()))
                    } else {
                        self.format_command_name(cmd.name())
                    };

                    println!("  {}{} - {}", formatted_name, alias_text, cmd.description());
                }
            }
        }
    }

    /// Display help information for a command
    pub fn display_help(&self, command: &dyn Command) {
        let name = command.name();
        let description = command.description();
        let aliases = command.aliases();
        let subcommands = command.subcommands();

        // Header
        if self.colored {
            println!("\x1b[1;32m{}\x1b[0m", name.to_uppercase());
            println!("{}", "=".repeat(name.len()));
        } else {
            println!("{}", name.to_uppercase());
            println!("{}", "=".repeat(name.len()));
        }

        // Description
        println!("{description}");

        // Aliases
        if !aliases.is_empty() {
            println!("\nAliases: {}", aliases.join(", "));
        }

        // Usage
        println!("\nUsage: {}", command.usage());

        // Subcommands
        if !subcommands.is_empty() {
            println!("\nSubcommands:");
            for subcmd in subcommands {
                // Skip the info command - it's a secret command
                if subcmd.name() == "info" {
                    continue;
                }

                let sub_aliases = subcmd.aliases();
                let alias_text = if sub_aliases.is_empty() {
                    String::new()
                } else {
                    let uppercase_aliases: Vec<String> =
                        sub_aliases.iter().map(|a| a.to_uppercase()).collect();
                    format!(" ({})", uppercase_aliases.join(", "))
                };

                let formatted_name = if self.colored {
                    format!(
                        "\x1b[1;36m{}\x1b[0m",
                        self.format_command_name(subcmd.name())
                    )
                } else {
                    self.format_command_name(subcmd.name())
                };

                println!(
                    "  {}{} - {}",
                    formatted_name,
                    alias_text,
                    subcmd.description()
                );
            }
        }
    }

    /// Display a success message
    pub fn display_success(&self, message: &str) {
        if !message.is_empty() {
            let icon = if self.unicode { "✓" } else { "OK" };
            let color = if self.colored { "\x1b[1;32m" } else { "" };
            let reset = if self.colored { "\x1b[0m" } else { "" };

            println!("{color}{icon} {message}{reset}");
        }
    }

    /// Display a warning message
    pub fn display_warning(&self, message: &str) {
        let icon = if self.unicode { "⚠" } else { "WARNING" };
        let color = if self.colored { "\x1b[1;33m" } else { "" };
        let reset = if self.colored { "\x1b[0m" } else { "" };

        println!("{color}{icon} {message}{reset}");
    }

    /// Display an informational message
    pub fn display_info(&self, message: &str) {
        let icon = if self.unicode { "ℹ" } else { "INFO" };
        let color = if self.colored { "\x1b[1;34m" } else { "" };
        let reset = if self.colored { "\x1b[0m" } else { "" };

        println!("{color}{icon} {message}{reset}");
    }

    /// Clear the terminal screen
    pub fn clear_screen(&self) -> io::Result<()> {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush()
    }

    /// Format a command name with bold alias character
    pub fn format_command_with_alias(&self, name: &str, alias: Option<&str>) -> String {
        if let Some(alias_char) = alias {
            if let Some(first_char) = name.chars().next() {
                if alias_char.starts_with(first_char) && self.colored {
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

    /// Format a command name with bold characters for display
    pub fn format_command_name(&self, name: &str) -> String {
        if !self.colored {
            return name.to_string();
        }

        // Special formatting for commands with aliases that match their first letter
        match name.to_lowercase().as_str() {
            "file" => {
                // Bold the 'F' in 'File' (alias: f)
                "\x1b[1mF\x1b[0mile".to_string()
            }
            "edit" => {
                // Bold the 'E' in 'Edit' (alias: e)
                "\x1b[1mE\x1b[0mdit".to_string()
            }
            "view" => {
                // Bold the 'V' in 'View' (alias: v)
                "\x1b[1mV\x1b[0miew".to_string()
            }
            "help" => {
                // Bold the 'H' in 'Help' (alias: h)
                "\x1b[1mH\x1b[0melp".to_string()
            }
            "quit" => {
                // Bold the 'Q' in 'Quit' (alias: q)
                "\x1b[1mQ\x1b[0muit".to_string()
            }
            "info" => {
                // Bold the 'I' in 'Info' (alias: i)
                "\x1b[1mI\x1b[0mnfo".to_string()
            }
            "load" => {
                // Bold the 'L' in 'Load' (alias: l)
                "\x1b[1mL\x1b[0moad".to_string()
            }
            "save" => {
                // Bold the 'S' in 'Save' (alias: s)
                "\x1b[1mS\x1b[0mave".to_string()
            }
            "exit" => {
                // Bold the 'E' in 'Exit' (alias: e)
                "\x1b[1mE\x1b[0mxit".to_string()
            }
            "axis" => {
                // Bold the 'A' in 'Axis' (alias: a)
                "\x1b[1mA\x1b[0mxis".to_string()
            }
            "show" => {
                // Bold the 'S' in 'Show' (alias: sh)
                "\x1b[1mS\x1b[0mhow".to_string()
            }
            "vers" => {
                // Bold the 'V' in 'Vers' (alias: v)
                "\x1b[1mV\x1b[0mers".to_string()
            }
            _ => name.to_string(),
        }
    }

    /// Display a progress indicator
    pub fn display_progress(&self, message: &str, current: usize, total: usize) {
        let percentage = if total > 0 {
            (current * 100) / total
        } else {
            0
        };

        let bar_width = 30;
        let filled = (percentage * bar_width) / 100;
        let empty = bar_width - filled;

        let bar = if self.unicode {
            format!("{}{}", "█".repeat(filled), "░".repeat(empty))
        } else {
            format!("{}{}", "=".repeat(filled), "-".repeat(empty))
        };

        print!("\r{message}: [{bar}] {percentage}% ({current}/{total})");
        io::stdout().flush().unwrap_or(());
    }

    /// Finish progress display
    pub fn finish_progress(&self) {
        println!();
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Terminal utilities for low-level terminal operations
pub struct TerminalUtils;

impl TerminalUtils {
    /// Check if stdout is a terminal (TTY)
    pub fn is_tty() -> bool {
        // This is a simplified check - in a full implementation,
        // you would use platform-specific code or a crate like `atty`
        std::env::var("TERM").is_ok()
    }

    /// Get terminal width
    pub fn get_width() -> usize {
        // Default width if we can't determine it
        80
    }

    /// Get terminal height
    pub fn get_height() -> usize {
        // Default height if we can't determine it
        24
    }

    /// Move cursor to position
    pub fn move_cursor(x: usize, y: usize) -> io::Result<()> {
        print!("\x1b[{y};{x}H");
        io::stdout().flush()
    }

    /// Hide cursor
    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1b[?25l");
        io::stdout().flush()
    }

    /// Show cursor
    pub fn show_cursor() -> io::Result<()> {
        print!("\x1b[?25h");
        io::stdout().flush()
    }

    /// Save cursor position
    pub fn save_cursor() -> io::Result<()> {
        print!("\x1b[s");
        io::stdout().flush()
    }

    /// Restore cursor position
    pub fn restore_cursor() -> io::Result<()> {
        print!("\x1b[u");
        io::stdout().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_manager_creation() {
        let dm = DisplayManager::new();
        assert!(dm.colored);
        assert!(dm.unicode);

        let dm = DisplayManager::with_options(false, false);
        assert!(!dm.colored);
        assert!(!dm.unicode);
    }

    #[test]
    fn test_format_command_with_alias() {
        let dm = DisplayManager::with_options(false, true);

        let result = dm.format_command_with_alias("help", Some("h"));
        assert_eq!(result, "help");

        let result = dm.format_command_with_alias("test", None);
        assert_eq!(result, "test");
    }

    #[test]
    fn test_format_command_name() {
        let dm = DisplayManager::with_options(true, true);

        // Test all main commands get bold first letter
        let result = dm.format_command_name("file");
        assert_eq!(result, "\x1b[1mF\x1b[0mile");

        let result = dm.format_command_name("edit");
        assert_eq!(result, "\x1b[1mE\x1b[0mdit");

        let result = dm.format_command_name("view");
        assert_eq!(result, "\x1b[1mV\x1b[0miew");

        let result = dm.format_command_name("help");
        assert_eq!(result, "\x1b[1mH\x1b[0melp");

        let result = dm.format_command_name("quit");
        assert_eq!(result, "\x1b[1mQ\x1b[0muit");

        let result = dm.format_command_name("info");
        assert_eq!(result, "\x1b[1mI\x1b[0mnfo");

        let result = dm.format_command_name("load");
        assert_eq!(result, "\x1b[1mL\x1b[0moad");

        let result = dm.format_command_name("save");
        assert_eq!(result, "\x1b[1mS\x1b[0mave");

        let result = dm.format_command_name("exit");
        assert_eq!(result, "\x1b[1mE\x1b[0mxit");

        let result = dm.format_command_name("axis");
        assert_eq!(result, "\x1b[1mA\x1b[0mxis");

        let result = dm.format_command_name("show");
        assert_eq!(result, "\x1b[1mS\x1b[0mhow");

        let result = dm.format_command_name("vers");
        assert_eq!(result, "\x1b[1mV\x1b[0mers");

        // Test case insensitive matching
        let result = dm.format_command_name("View");
        assert_eq!(result, "\x1b[1mV\x1b[0miew");

        let result = dm.format_command_name("FILE");
        assert_eq!(result, "\x1b[1mF\x1b[0mile");

        // Test unknown commands remain unchanged
        let result = dm.format_command_name("unknown");
        assert_eq!(result, "unknown");

        // Test with colors disabled
        let dm_no_color = DisplayManager::with_options(false, true);
        let result = dm_no_color.format_command_name("view");
        assert_eq!(result, "view");

        let result = dm_no_color.format_command_name("file");
        assert_eq!(result, "file");
    }

    #[test]
    fn test_terminal_utils() {
        assert_eq!(TerminalUtils::get_width(), 80);
        assert_eq!(TerminalUtils::get_height(), 24);
    }
}
