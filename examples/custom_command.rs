//! Example of creating a custom command for sm-menu
//!
//! This example demonstrates how to create a custom command that can be
//! integrated into the sm-menu application.
//!
//! To run this example:
//! ```bash
//! cargo run --example custom_command
//! ```

use sm_menu::{CliContext, CliError, CliResult, Command, CommandResult};

/// A custom hello command that greets the user
#[derive(Debug)]
struct HelloCommand;

impl HelloCommand {
    fn new() -> Self {
        Self
    }
}

impl Command for HelloCommand {
    fn name(&self) -> &str {
        "hello"
    }

    fn description(&self) -> &str {
        "Greet the user with a friendly message"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["hi", "greet"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        // Handle arguments
        match args.len() {
            0 => {
                // No arguments - generic greeting
                println!("\nHello! Welcome to sm-menu!");
                println!("Type 'help' for available commands.\n");
                Ok(CommandResult::Success(String::new()))
            }
            1 => {
                // One argument - personalized greeting
                let name = &args[0];
                println!("\nHello, {}! Nice to meet you!", name);
                println!("Welcome to sm-menu!\n");
                Ok(CommandResult::Success(String::new()))
            }
            _ => {
                // Too many arguments - return error
                Err(CliError::TooManyArguments {
                    expected: 1,
                    found: args.len(),
                })
            }
        }
    }

    fn has_subcommands(&self) -> bool {
        false
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        Vec::new()
    }

    fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        input_lower == self.name() || self.aliases().iter().any(|a| input_lower == *a)
    }
}

/// A custom calculator command with subcommands
#[derive(Debug)]
struct CalculatorCommand;

impl CalculatorCommand {
    fn new() -> Self {
        Self
    }
}

impl Command for CalculatorCommand {
    fn name(&self) -> &str {
        "calc"
    }

    fn description(&self) -> &str {
        "Simple calculator operations"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["calculator"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        if !args.is_empty() {
            return Err(CliError::TooManyArguments {
                expected: 0,
                found: args.len(),
            });
        }
        Ok(CommandResult::Continue)
    }

    fn has_subcommands(&self) -> bool {
        true
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(AddCommand::new()),
            Box::new(SubtractCommand::new()),
        ]
    }

    fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        input_lower == self.name() || self.aliases().iter().any(|a| input_lower == *a)
    }
}

/// Addition subcommand
#[derive(Debug)]
struct AddCommand;

impl AddCommand {
    fn new() -> Self {
        Self
    }
}

impl Command for AddCommand {
    fn name(&self) -> &str {
        "add"
    }

    fn description(&self) -> &str {
        "Add two numbers"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["+"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        if args.len() != 2 {
            return Err(CliError::execution_error(&format!(
                "Expected 2 arguments, got {}. Usage: add <num1> <num2>",
                args.len()
            )));
        }

        let a: f64 = args[0]
            .parse()
            .map_err(|_| CliError::invalid_input(&format!("'{}' is not a valid number", args[0])))?;
        let b: f64 = args[1]
            .parse()
            .map_err(|_| CliError::invalid_input(&format!("'{}' is not a valid number", args[1])))?;

        let result = a + b;
        Ok(CommandResult::Success(format!("{} + {} = {}", a, b, result)))
    }

    fn has_subcommands(&self) -> bool {
        false
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        Vec::new()
    }

    fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        input_lower == self.name() || self.aliases().iter().any(|a| input_lower == *a)
    }
}

/// Subtraction subcommand
#[derive(Debug)]
struct SubtractCommand;

impl SubtractCommand {
    fn new() -> Self {
        Self
    }
}

impl Command for SubtractCommand {
    fn name(&self) -> &str {
        "subtract"
    }

    fn description(&self) -> &str {
        "Subtract two numbers"
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["-", "sub"]
    }

    fn execute(&mut self, args: &[String]) -> CliResult<CommandResult> {
        if args.len() != 2 {
            return Err(CliError::execution_error(&format!(
                "Expected 2 arguments, got {}. Usage: subtract <num1> <num2>",
                args.len()
            )));
        }

        let a: f64 = args[0]
            .parse()
            .map_err(|_| CliError::invalid_input(&format!("'{}' is not a valid number", args[0])))?;
        let b: f64 = args[1]
            .parse()
            .map_err(|_| CliError::invalid_input(&format!("'{}' is not a valid number", args[1])))?;

        let result = a - b;
        Ok(CommandResult::Success(format!("{} - {} = {}", a, b, result)))
    }

    fn has_subcommands(&self) -> bool {
        false
    }

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        Vec::new()
    }

    fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        input_lower == self.name() || self.aliases().iter().any(|a| input_lower == *a)
    }
}

fn main() {
    println!("Custom Command Example\n");
    println!("This example demonstrates how to create custom commands.\n");

    // Test the HelloCommand
    println!("=== Testing HelloCommand ===\n");
    let mut hello_cmd = HelloCommand::new();

    println!("Command: {}", hello_cmd.name());
    println!("Description: {}", hello_cmd.description());
    println!("Aliases: {:?}\n", hello_cmd.aliases());

    // Execute with no arguments
    println!("Executing: hello");
    if let Ok(CommandResult::Success(_)) = hello_cmd.execute(&[]) {
        println!("Success!\n");
    }

    // Execute with one argument
    println!("Executing: hello Alice");
    if let Ok(CommandResult::Success(_)) = hello_cmd.execute(&[String::from("Alice")]) {
        println!("Success!\n");
    }

    // Execute with too many arguments (should error)
    println!("Executing: hello Alice Bob (should error)");
    match hello_cmd.execute(&[String::from("Alice"), String::from("Bob")]) {
        Err(e) => println!("Error (expected): {}\n", e),
        Ok(_) => println!("Unexpected success!\n"),
    }

    // Test the CalculatorCommand
    println!("=== Testing CalculatorCommand ===\n");
    let calc_cmd = CalculatorCommand::new();

    println!("Command: {}", calc_cmd.name());
    println!("Description: {}", calc_cmd.description());
    println!("Has subcommands: {}\n", calc_cmd.has_subcommands());

    // Test the AddCommand
    println!("=== Testing AddCommand ===\n");
    let mut add_cmd = AddCommand::new();

    println!("Executing: add 10 5");
    match add_cmd.execute(&[String::from("10"), String::from("5")]) {
        Ok(CommandResult::Success(msg)) => println!("{}\n", msg),
        Ok(_) => println!("Unexpected result\n"),
        Err(e) => println!("Error: {}\n", e),
    }

    // Test the SubtractCommand
    println!("=== Testing SubtractCommand ===\n");
    let mut sub_cmd = SubtractCommand::new();

    println!("Executing: subtract 10 5");
    match sub_cmd.execute(&[String::from("10"), String::from("5")]) {
        Ok(CommandResult::Success(msg)) => println!("{}\n", msg),
        Ok(_) => println!("Unexpected result\n"),
        Err(e) => println!("Error: {}\n", e),
    }

    println!("=== Integration Example ===\n");
    println!("To integrate these commands into sm-menu:");
    println!("1. Add them to the RootCommand's subcommands() method");
    println!("2. Recompile the application");
    println!("3. Run sm-menu and use your custom commands!\n");

    println!("Example code:");
    println!("```rust");
    println!("fn subcommands(&self) -> Vec<Box<dyn Command>> {{");
    println!("    vec![");
    println!("        Box::new(HelloCommand::new()),");
    println!("        Box::new(CalculatorCommand::new()),");
    println!("        // ... other commands");
    println!("    ]");
    println!("}}");
    println!("```");
}
