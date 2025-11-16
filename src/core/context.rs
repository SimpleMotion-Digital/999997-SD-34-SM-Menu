//! CLI context management module.
//!
//! This module provides the context management system for the CLI application,
//! handling navigation state, command history, and user session management.

use crate::core::command::Command;
use std::collections::VecDeque;

/// Maximum number of commands to keep in history
const MAX_HISTORY_SIZE: usize = 100;

/// CLI context for managing application state and navigation
///
/// This struct maintains the current state of the CLI application including
/// navigation context, command history, and user preferences.
#[derive(Debug)]
pub struct CliContext {
    /// Current navigation path (stack of menu names)
    current_path: Vec<String>,
    /// Whether the application is still running
    pub running: bool,
    /// Command history for user convenience
    history: VecDeque<String>,
    /// Current history position (for history navigation)
    history_position: usize,
    /// User preferences
    preferences: CliPreferences,
}

impl CliContext {
    /// Create a new CLI context with default settings
    pub fn new() -> Self {
        Self {
            current_path: Vec::new(),
            running: true,
            history: VecDeque::new(),
            history_position: 0,
            preferences: CliPreferences::default(),
        }
    }

    /// Push a new context level (enter submenu)
    pub fn push_context(&mut self, name: String) {
        self.current_path.push(name);
    }

    /// Pop the current context level (exit to parent menu)
    pub fn pop_context(&mut self) -> Option<String> {
        self.current_path.pop()
    }

    /// Get the current navigation path
    pub fn current_path(&self) -> &[String] {
        &self.current_path
    }

    /// Get the current depth in the navigation hierarchy
    pub fn depth(&self) -> usize {
        self.current_path.len()
    }

    /// Check if we're at the root level
    pub fn is_root(&self) -> bool {
        self.current_path.is_empty()
    }

    /// Get the formatted prompt string
    pub fn get_prompt(&self) -> String {
        // Use Warp-like green color (24-bit color for better accuracy)
        let green_color = if self.preferences.colored_prompt {
            "\x1b[38;2;0;215;135m"
        } else {
            ""
        };
        let reset_color = if self.preferences.colored_prompt {
            "\x1b[0m"
        } else {
            ""
        };

        if self.current_path.is_empty() {
            format!("{green_color}sm-menu{reset_color} > ")
        } else {
            format!(
                "{}sm-menu{} ~ {} > ",
                green_color,
                reset_color,
                self.current_path.join(" > ")
            )
        }
    }

    /// Signal that the application should quit
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Add a command to the history
    pub fn add_to_history(&mut self, command: String) {
        if !command.trim().is_empty() && self.history.back() != Some(&command) {
            self.history.push_back(command);

            // Limit history size
            if self.history.len() > MAX_HISTORY_SIZE {
                self.history.pop_front();
            }

            // Reset history position
            self.history_position = self.history.len();
        }
    }

    /// Get the command history
    pub fn history(&self) -> &VecDeque<String> {
        &self.history
    }

    /// Get the previous command in history
    pub fn previous_command(&mut self) -> Option<&String> {
        if self.history_position > 0 {
            self.history_position -= 1;
            self.history.get(self.history_position)
        } else {
            None
        }
    }

    /// Get the next command in history
    pub fn next_command(&mut self) -> Option<&String> {
        if self.history_position < self.history.len().saturating_sub(1) {
            self.history_position += 1;
            self.history.get(self.history_position)
        } else {
            None
        }
    }

    /// Get command completions for the given prefix
    pub fn get_completions(
        &self,
        prefix: &str,
        available_commands: &[Box<dyn Command>],
    ) -> Vec<String> {
        let mut completions = Vec::new();

        // Add command name completions
        for cmd in available_commands {
            if cmd.name().starts_with(prefix) {
                completions.push(cmd.name().to_string());
            }

            // Add alias completions
            for alias in cmd.aliases() {
                if alias.starts_with(prefix) {
                    completions.push(alias.to_string());
                }
            }
        }

        // Add history completions
        for command in &self.history {
            if command.starts_with(prefix) && !completions.contains(command) {
                completions.push(command.clone());
            }
        }

        completions.sort();
        completions.dedup();
        completions
    }

    /// Get user preferences
    pub fn preferences(&self) -> &CliPreferences {
        &self.preferences
    }

    /// Get mutable user preferences
    pub fn preferences_mut(&mut self) -> &mut CliPreferences {
        &mut self.preferences
    }

    /// Reset the context to initial state
    pub fn reset(&mut self) {
        self.current_path.clear();
        self.running = true;
        self.history_position = self.history.len();
    }
}

impl Default for CliContext {
    fn default() -> Self {
        Self::new()
    }
}

/// User preferences for CLI behavior
#[derive(Debug, Clone)]
pub struct CliPreferences {
    /// Whether to use colored output
    pub colored_prompt: bool,
    /// Whether to show command suggestions on invalid input
    pub show_suggestions: bool,
    /// Whether to confirm destructive operations
    pub confirm_destructive: bool,
    /// Maximum number of items to show in listings
    pub max_list_items: usize,
}

impl Default for CliPreferences {
    fn default() -> Self {
        Self {
            colored_prompt: true,
            show_suggestions: true,
            confirm_destructive: true,
            max_list_items: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = CliContext::new();
        assert!(context.running);
        assert!(context.current_path.is_empty());
        assert!(context.is_root());
        assert_eq!(context.depth(), 0);
    }

    #[test]
    fn test_context_navigation() {
        let mut context = CliContext::new();

        // Test pushing context
        context.push_context("file".to_string());
        assert_eq!(context.current_path(), &["file"]);
        assert_eq!(context.depth(), 1);
        assert!(!context.is_root());

        // Test popping context
        let popped = context.pop_context();
        assert_eq!(popped, Some("file".to_string()));
        assert!(context.is_root());
        assert_eq!(context.depth(), 0);
    }

    #[test]
    fn test_prompt_generation() {
        let mut context = CliContext::new();
        context.preferences.colored_prompt = false;

        // Root prompt
        assert_eq!(context.get_prompt(), "sm-menu > ");

        // Nested prompt
        context.push_context("file".to_string());
        assert_eq!(context.get_prompt(), "sm-menu ~ file > ");

        context.push_context("load".to_string());
        assert_eq!(context.get_prompt(), "sm-menu ~ file > load > ");
    }

    #[test]
    fn test_colored_prompt_generation() {
        let mut context = CliContext::new();
        context.preferences.colored_prompt = true;

        // Root prompt with Warp-style green color
        assert_eq!(
            context.get_prompt(),
            "\x1b[38;2;0;215;135msm-menu\x1b[0m > "
        );

        // Nested prompt with Warp-style green color
        context.push_context("file".to_string());
        assert_eq!(
            context.get_prompt(),
            "\x1b[38;2;0;215;135msm-menu\x1b[0m ~ file > "
        );
    }

    #[test]
    fn test_history_management() {
        let mut context = CliContext::new();

        // Add commands to history
        context.add_to_history("command1".to_string());
        context.add_to_history("command2".to_string());
        context.add_to_history("command3".to_string());

        assert_eq!(context.history().len(), 3);

        // Test duplicate prevention
        context.add_to_history("command3".to_string());
        assert_eq!(context.history().len(), 3);

        // Test empty command prevention
        context.add_to_history("".to_string());
        context.add_to_history("   ".to_string());
        assert_eq!(context.history().len(), 3);
    }
}
