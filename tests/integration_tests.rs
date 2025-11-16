//! Integration tests for robust error handling
//!
//! This module contains tests that verify the error handling behavior
//! of various commands in the CLI application.

use sm_menu::commands::file::FileCommand;
use sm_menu::commands::help::HelpCommand;
use sm_menu::commands::quit::QuitCommand;
use sm_menu::commands::vers::VersCommand;
use sm_menu::{CliError, Command, CommandResult};

#[test]
fn test_vers_command_error_handling() {
    let mut cmd = VersCommand::new();

    // Test with no arguments (should succeed)
    let result = cmd.execute(&[]);
    assert!(result.is_ok());

    // Test with too many arguments (should fail)
    let result = cmd.execute(&["arg1".to_string(), "arg2".to_string()]);
    assert!(result.is_err());

    if let Err(CliError::TooManyArguments { expected, found }) = result {
        assert_eq!(expected, 0);
        assert_eq!(found, 2);
    } else {
        panic!("Expected TooManyArguments error");
    }
}

#[test]
fn test_help_command_error_handling() {
    let mut cmd = HelpCommand::new();

    // Test with no arguments (should succeed)
    let result = cmd.execute(&[]);
    assert!(result.is_ok());

    // Test with valid command name (should succeed)
    let result = cmd.execute(&["file".to_string()]);
    assert!(result.is_ok());

    // Test with invalid command name (should fail)
    let result = cmd.execute(&["invalid".to_string()]);
    assert!(result.is_err());

    if let Err(CliError::InvalidInput(_)) = result {
        // Expected error type
    } else {
        panic!("Expected InvalidInput error");
    }

    // Test with too many arguments (should fail)
    let result = cmd.execute(&["file".to_string(), "extra".to_string()]);
    assert!(result.is_err());

    if let Err(CliError::TooManyArguments { expected, found }) = result {
        assert_eq!(expected, 1);
        assert_eq!(found, 2);
    } else {
        panic!("Expected TooManyArguments error");
    }
}

#[test]
fn test_quit_command_error_handling() {
    let mut cmd = QuitCommand::new();

    // Test with no arguments (should succeed)
    let result = cmd.execute(&[]);
    assert!(result.is_ok());

    if let Ok(CommandResult::Quit) = result {
        // Expected result
    } else {
        panic!("Expected Quit result");
    }

    // Test with too many arguments (should fail)
    let result = cmd.execute(&["arg1".to_string()]);
    assert!(result.is_err());

    if let Err(CliError::TooManyArguments { expected, found }) = result {
        assert_eq!(expected, 0);
        assert_eq!(found, 1);
    } else {
        panic!("Expected TooManyArguments error");
    }
}

#[test]
fn test_file_command_error_handling() {
    let mut cmd = FileCommand::new();

    // Test with no arguments (should succeed)
    let result = cmd.execute(&[]);
    assert!(result.is_ok());

    // Test with too many arguments (should fail)
    let result = cmd.execute(&["arg1".to_string()]);
    assert!(result.is_err());

    if let Err(CliError::TooManyArguments { expected, found }) = result {
        assert_eq!(expected, 0);
        assert_eq!(found, 1);
    } else {
        panic!("Expected TooManyArguments error");
    }
}
