# SM-Menu Usage Guide

## Table of Contents
- [Getting Started](#getting-started)
- [Basic Commands](#basic-commands)
- [Navigation](#navigation)
- [File Operations](#file-operations)
- [Tips and Tricks](#tips-and-tricks)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Running sm-menu

After installation, simply run:

```bash
sm-menu
```

You'll be greeted with the interactive menu interface:

```
Welcome to sm-menu!

sm-menu >
```

### Getting Help

At any prompt, you can:

- Type `help` to see available commands
- Type `help <command>` to get detailed information about a specific command
- Press Enter (empty input) to see available commands at the current level

## Basic Commands

### Help Command

Display help information for commands:

```bash
sm-menu > help
```

Get help for a specific command:

```bash
sm-menu > help file
```

### Version Command

Display the current version:

```bash
sm-menu > vers
```

Aliases: `v`, `version`

### Quit Command

Exit the application:

```bash
sm-menu > quit
```

Aliases: `q`, `exit`

## Navigation

SM-menu uses a hierarchical menu system. You can navigate through menus to access different features.

### Entering a Submenu

Simply type the name of a submenu command:

```bash
sm-menu > file
sm-menu ~ file >
```

Notice how the prompt changes to show your current location (`~ file`).

### Going Back

Use the `exit` command to return to the parent menu:

```bash
sm-menu ~ file > exit
sm-menu >
```

### Navigation Depth

The system allows navigation up to 10 levels deep to prevent stack overflow.

## File Operations

### Accessing File Menu

```bash
sm-menu > file
sm-menu ~ file >
```

### Load Command

Load a file:

```bash
sm-menu ~ file > load myfile.txt
```

### Save Command

Save data to a file:

```bash
sm-menu ~ file > save output.txt
```

### View Command

View file contents:

```bash
sm-menu ~ file > view document.txt
```

### Edit Command

Edit a file (placeholder for future implementation):

```bash
sm-menu ~ file > edit config.txt
```

## Tips and Tricks

### Command Aliases

Most commands have shorter aliases for faster typing:

- `help` → `h`, `?`
- `file` → `f`
- `quit` → `q`, `exit`
- `vers` → `v`, `version`

### Case Insensitive

All commands are case-insensitive:

```bash
sm-menu > HELP
sm-menu > Help
sm-menu > help
```

All three work the same way!

### Empty Input

Pressing Enter without typing anything shows available commands:

```bash
sm-menu > [press Enter]

  file (F) - File operations
  help (H, ?) - Display help information
  show (S) - Display information
  vers (V, VERSION) - Display version information
  quit (Q, EXIT) - Exit the application
```

### Command Discovery

Use `help` to discover available commands:

```bash
# List all commands
sm-menu > help

# Get detailed help for a command
sm-menu > help file
```

## Troubleshooting

### Command Not Found

If you see "Invalid command: 'xyz'", check:

1. Is the command spelled correctly?
2. Are you in the right menu level?
3. Type `help` to see available commands

### Maximum Navigation Depth

If you see "Maximum navigation depth reached":

- You've navigated too deep (10 levels)
- Use `exit` to go back to a higher level
- This prevents stack overflow

### File Access Issues

When working with files:

- Ensure files exist before trying to load them
- Check file permissions
- Use relative paths from current working directory
- Path traversal attacks (`../`) are prevented for security

## Examples

### Example 1: Basic Navigation

```bash
$ sm-menu

sm-menu > help
[Shows list of commands]

sm-menu > file
sm-menu ~ file > help
[Shows file commands]

sm-menu ~ file > exit
sm-menu > quit
```

### Example 2: File Operations

```bash
$ sm-menu

sm-menu > file
sm-menu ~ file > load data.txt
[File contents loaded]

sm-menu ~ file > save output.txt
[Data saved]

sm-menu ~ file > exit
sm-menu > quit
```

### Example 3: Getting Version Info

```bash
$ sm-menu

sm-menu > vers
sm-menu version 0.1.0

sm-menu > quit
```

### Example 4: Using Aliases

```bash
$ sm-menu

sm-menu > f          # Enter file menu
sm-menu ~ file > ?   # Show help
sm-menu ~ file > q   # Quit
```

## Advanced Usage

### Context Awareness

The application maintains context as you navigate:

```bash
sm-menu > file
sm-menu ~ file > axis
sm-menu ~ file > axis > show
[Shows axis information]
```

### Error Messages

The application provides helpful error messages:

```bash
sm-menu > invalid
✗ Invalid command: 'invalid'

  file (F) - File operations
  help (H, ?) - Display help information
  ...
```

## Keyboard Shortcuts

Currently, sm-menu uses standard terminal input:

- `Ctrl+C`: Interrupt operation (shows "Operation interrupted" message)
- `Ctrl+D`: End of input (may exit application depending on terminal settings)

## Configuration

User preferences can be modified through the `CliPreferences` struct when using sm-menu as a library:

- `colored_prompt`: Enable/disable colored output
- `show_suggestions`: Enable/disable command suggestions
- `confirm_destructive`: Enable/disable confirmation for destructive operations
- `max_list_items`: Maximum items to show in listings

## Using as a Library

SM-menu can also be used as a library in your Rust projects. See the `examples/` directory for custom command implementations.

## Getting Help

For more information:

- GitHub Repository: https://github.com/SimpleMotion-Digital/999997-SD-34-SM-Menu
- Issues: https://github.com/SimpleMotion-Digital/999997-SD-34-SM-Menu/issues
- Documentation: Run `cargo doc --open` in the project directory
