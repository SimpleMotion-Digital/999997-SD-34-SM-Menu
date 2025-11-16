pub mod commands;
pub mod core;
pub mod ui;

#[cfg(test)]
mod integration_tests;

// Re-export for tests
pub use commands::*;
pub use core::*;
pub use ui::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_context_creation() {
        let context = CliContext::new();
        assert!(context.running);
        assert!(context.current_path().is_empty());
    }

    #[test]
    fn test_cli_context_navigation() {
        let mut context = CliContext::new();

        // Disable colored prompts for testing
        context.preferences_mut().colored_prompt = false;

        // Test pushing context
        context.push_context("file".to_string());
        assert_eq!(context.current_path(), vec!["file"]);
        assert_eq!(context.get_prompt(), "sm-menu ~ file > ");

        // Test popping context
        context.pop_context();
        assert!(context.current_path().is_empty());
        assert_eq!(context.get_prompt(), "sm-menu > ");
    }

    #[test]
    fn test_error_display() {
        let error = CliError::InvalidCommand("test".to_string());
        assert_eq!(format!("{error}"), "Invalid command: 'test'");

        let error = CliError::EmptyInput;
        assert_eq!(format!("{error}"), "Empty input provided");

        let error = CliError::TooManyArguments {
            expected: 2,
            found: 5,
        };
        assert_eq!(
            format!("{error}"),
            "Too many arguments: expected 2, found 5"
        );

        let error = CliError::TooFewArguments {
            expected: 2,
            found: 1,
        };
        assert_eq!(format!("{error}"), "Too few arguments: expected 2, found 1");

        let error = CliError::ExecutionError("command failed".to_string());
        assert_eq!(
            format!("{error}"),
            "Command execution failed: command failed"
        );
    }
}
