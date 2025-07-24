# simple-menu

A simple interactive CLI tool for menu management using only Rust's standard library.

## Overview

`simple-menu` is a Rust CLI application that provides an interactive menu system for various operations. The project emphasizes simplicity and uses only the Rust standard library without external dependencies.

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-username/simple-menu.git
cd simple-menu

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

## Usage

```bash
# Run the interactive menu
simple-menu
```

The application provides an interactive command-line interface with various menu options and commands.

## Features

- **Interactive Menu System**: Navigate through hierarchical menu structures
- **Standard Library Only**: No external dependencies - uses only Rust `std`
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Extensible**: Modular command architecture for easy extension
- **Error Handling**: Comprehensive error handling and user feedback

## Commands

The application supports various commands accessible through the interactive menu:

- `help` - Display available commands and usage information
- `show` - Display current menu or system information
- `view` - View files or data in different formats
- `edit` - Edit configuration or data files
- `file` - File operations and management
- `load` - Load data or configuration files
- `save` - Save current state or data
- `quit` - Exit the application

## Development

### Prerequisites

- Rust 2024 edition or later
- Cargo (comes with Rust)

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library interface
├── commands/            # Command implementations
│   ├── mod.rs          # Command module exports
│   ├── help.rs         # Help command
│   ├── show.rs         # Show command
│   ├── view.rs         # View command
│   ├── edit.rs         # Edit command
│   ├── file.rs         # File command
│   ├── load.rs         # Load command
│   ├── save.rs         # Save command
│   └── quit.rs         # Quit command
├── core/                # Core functionality
│   ├── mod.rs          # Core module exports
│   ├── command.rs      # Command trait and types
│   ├── context.rs      # Application context
│   └── error.rs        # Error types and handling
└── ui/                  # User interface
    ├── mod.rs          # UI module exports
    └── disp.rs         # Display management
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and ensure they pass
5. Format and lint your code
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
