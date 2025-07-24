use simple_menu::ui::DisplayManager;
use simple_menu::{CliContext, CliError, CliResult, Command, CommandResult};
use std::io::{self, BufRead, Write};
use std::panic;
use std::process;

mod commands;
use commands::RootCommand;

/// Clear the terminal screen using ANSI escape codes
fn clear_terminal() -> CliResult<()> {
    // ANSI escape code to clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[H");
    io::stdout()
        .flush()
        .map_err(|e| CliError::terminal_error(&format!("Failed to clear terminal: {e}")))?;
    Ok(())
}

fn main() -> CliResult<()> {
    // Set up panic handler for graceful error handling
    setup_panic_handler();

    // Set up signal handlers for graceful shutdown
    setup_signal_handlers();

    // Clear the terminal screen
    if let Err(e) = clear_terminal() {
        eprintln!("Warning: {e}");
    }

    println!("\n\tWelcome to simple-menu!\n");

    let mut context = CliContext::new();
    let mut command_stack: Vec<Box<dyn Command>> = vec![Box::new(RootCommand)];

    // Main application loop with comprehensive error handling
    let result = run_main_loop(&mut context, &mut command_stack);

    // Perform graceful shutdown
    graceful_shutdown();

    result
}

/// Set up panic handler for better error reporting
fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("\nFatal error occurred!");
        eprintln!("Error: {panic_info}");
        eprintln!("Please report this bug to the developers.");
        process::exit(1);
    }));
}

/// Set up signal handlers for graceful shutdown (placeholder for now)
fn setup_signal_handlers() {
    // In a real implementation, you would set up signal handlers here
    // For now, we'll rely on Rust's default behavior
}

/// Main application loop with comprehensive error handling
fn run_main_loop(
    context: &mut CliContext,
    command_stack: &mut Vec<Box<dyn Command>>,
) -> CliResult<()> {
    while context.running {
        // Display prompt and read input
        match display_flashing_prompt_and_read_input(context) {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    // Show available commands when user presses enter with no input
                    show_available_commands(command_stack);
                    continue;
                }

                // Handle the input with comprehensive error handling
                if let Err(e) = handle_input(input, command_stack, context) {
                    display_error(&e, command_stack);
                }
            }
            Err(CliError::Interrupted) => {
                println!("\nOperation interrupted. Type 'quit' to exit.");
                continue;
            }
            Err(e) => {
                eprintln!("Error reading input: {e}");
                // Don't break on IO errors, try to continue
                continue;
            }
        }
    }

    Ok(())
}

/// Display prompt and read input (simplified without flashing animation)
fn display_flashing_prompt_and_read_input(context: &CliContext) -> CliResult<String> {
    let base_prompt = context.get_prompt();
    let question_mark = "? ";

    // Display static prompt
    print!("{base_prompt}{question_mark}");
    io::stdout()
        .flush()
        .map_err(|e| CliError::terminal_error(&format!("Failed to display prompt: {e}")))?;

    // Read input directly
    let stdin = io::stdin();
    let mut input = String::new();
    match stdin.lock().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => {
            let cli_error = match e.kind() {
                std::io::ErrorKind::Interrupted => CliError::Interrupted,
                _ => CliError::from(e),
            };
            Err(cli_error)
        }
    }
}

/// Show available commands when user presses enter with no input
fn show_available_commands(command_stack: &[Box<dyn Command>]) {
    if let Some(_current_command) = command_stack.last() {
        println!();
        let display_manager = DisplayManager::new();
        display_manager.display_available_commands(command_stack);
        println!();
    }
}

/// Display error with appropriate formatting
fn display_error(error: &CliError, command_stack: &[Box<dyn Command>]) {
    let display_manager = DisplayManager::new();
    display_manager.display_error(error, command_stack);
}

/// Perform graceful shutdown
fn graceful_shutdown() {
    println!("\nThank you for using simple-menu!");
    // Ensure stdout is flushed before exit
    let _ = io::stdout().flush();
}

fn handle_input(
    input: &str,
    command_stack: &mut Vec<Box<dyn Command>>,
    context: &mut CliContext,
) -> CliResult<()> {
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    if parts.is_empty() {
        return Err(CliError::EmptyInput);
    }

    let command_name = &parts[0];
    let args = &parts[1..];

    // Get current command level with error handling
    let current_command = command_stack
        .last()
        .ok_or_else(|| CliError::internal_error("Empty command stack"))?;
    let subcommands = current_command.subcommands();

    // Find matching command (by name or alias)
    let found_command = subcommands
        .into_iter()
        .find(|cmd| cmd.matches(command_name));

    match found_command {
        Some(mut cmd) => {
            // Execute the command with proper error handling
            match cmd.execute(args) {
                Ok(result) => {
                    match result {
                        CommandResult::Success(msg) => {
                            if !msg.is_empty() {
                                println!("{msg}");
                            }
                        }
                        CommandResult::Continue => {
                            // If command has subcommands, enter that submenu
                            if cmd.has_subcommands() {
                                context.push_context(cmd.name().to_string());
                                command_stack.push(cmd);
                            }
                        }
                        CommandResult::GoUp => {
                            // Return to parent menu
                            if command_stack.len() > 1 {
                                command_stack.pop();
                                context.pop_context();
                            } else {
                                // Already at root level
                                println!("Already at root level.");
                            }
                        }
                        CommandResult::Quit => {
                            context.quit();
                        }
                    }
                }
                Err(e) => {
                    // Return the error to be handled by the caller
                    return Err(e);
                }
            }
        }
        None => {
            // Command not found - this is now an error
            return Err(CliError::invalid_command(command_name));
        }
    }

    Ok(())
}
