# CLAUDE.md - Project Context

## Project Overview
999997-SD-34-SM-Menu is a Rust CLI application for interactive menu management, targeting release in 2024. The project follows a strict constraint of using only the Rust standard library (`std`) without external dependencies.

## Technology Stack
- **Language**: Rust (2024 edition)
- **Dependencies**: Rust standard library only (`std`)
- **Target**: CLI application
- **Platform**: Cross-platform (focusing on macOS development environment)

## Key Constraints
1. **No External Dependencies**: This project must use only the Rust standard library
2. **Std-lib Only**: All functionality must be implemented using `std` components
3. **2024 Release**: Target modern Rust practices and features available in 2024

## Project Structure
- CLI application architecture
- Modular design using Rust modules
- Standard library utilities for file I/O, networking, collections, etc.

## Development Environment
- **OS**: macOS
- **Shell**: zsh 5.9
- **Project Path**: `/Users/greg-gowans/Source/SD-Source/999997-SD-34-SM-Menu`
- **Repository**: `https://github.com/SimpleMotion-Digital/999997-SD-34-SM-Menu`
- **Binary Name**: `sm-menu`

## Rust Edition and Features
- Use Rust 2024 edition (latest stable)
- Leverage modern Rust features available in the standard library
- Focus on memory safety, performance, and ergonomics

## CLI Design Principles
- Clean, intuitive command-line interface
- Proper error handling using `Result` types
- Comprehensive help system
- Cross-platform compatibility

## Software Engineering Best Practices

### Code Quality
- **Rust Idioms**: Follow Rust naming conventions and idiomatic patterns
- **Error Handling**: Use `Result<T, E>` and `Option<T>` appropriately
- **Memory Safety**: Leverage Rust's ownership system and borrow checker
- **Type Safety**: Use strong typing and avoid `unsafe` code unless absolutely necessary
- **Documentation**: Document all public APIs with `///` comments
- **Code Comments**: Use `//` for implementation details and complex logic

### Testing Strategy
- **Unit Tests**: Test individual functions and modules using `#[cfg(test)]`
- **Integration Tests**: Place integration tests in `tests/` directory
- **Documentation Tests**: Include examples in documentation that are tested
- **Edge Cases**: Test boundary conditions and error scenarios
- **Test Coverage**: Aim for comprehensive test coverage of critical paths

### Code Organization
- **Modular Design**: Separate concerns into logical modules
- **Single Responsibility**: Each function/module should have one clear purpose
- **DRY Principle**: Avoid code duplication through proper abstraction
- **Clear Interfaces**: Design clean, minimal public APIs
- **Separation of Concerns**: Keep CLI parsing, business logic, and I/O separate

### Performance Considerations
- **Minimize Allocations**: Use string slices (`&str`) over `String` when possible
- **Efficient Collections**: Choose appropriate collection types (`Vec`, `HashMap`, etc.)
- **Lazy Evaluation**: Use iterators and closures for efficient processing
- **Resource Management**: Properly handle file handles and system resources
- **Profiling**: Use `std::time` for basic performance measurement

### Security Practices
- **Input Validation**: Validate all user inputs and command-line arguments
- **Path Traversal**: Prevent directory traversal attacks in file operations
- **Safe Parsing**: Use safe string parsing methods
- **Error Information**: Avoid exposing sensitive information in error messages
- **Resource Limits**: Implement reasonable limits on resource usage

### Maintainability
- **Consistent Formatting**: Use `cargo fmt` for code formatting
- **Linting**: Use `cargo clippy` for code quality checks
- **Version Control**: Use meaningful commit messages and atomic commits
- **Refactoring**: Regularly refactor to improve code quality
- **Dependencies**: Since we're std-lib only, carefully evaluate any std features used

### Development Workflow
- **Incremental Development**: Build and test incrementally
- **Code Reviews**: Review code changes before integration
- **Continuous Integration**: Ensure code builds and tests pass
- **Documentation**: Keep documentation up-to-date with code changes
- **Versioning**: Use semantic versioning for releases
- **Changelog Management**: **ALWAYS** update CHANGELOG.md with every code change, feature addition, bug fix, or improvement

### Changelog Requirements
- **Mandatory Updates**: Every code change, no matter how small, MUST be documented in CHANGELOG.md
- **Four-Part Versioning for CHANGELOG**: Use four-part version numbering system in CHANGELOG.md: `MAJOR.MINOR.PATCH.BUILD` (e.g., 0.1.2.5, 0.1.2.6, etc.)
- **Three-Part Versioning for Cargo.toml**: Cargo.toml MUST always use standard three-part semantic versioning: `MAJOR.MINOR.PATCH` (e.g., 0.1.2), even when CHANGELOG.md uses four-part versioning
- **Sequential Build Numbers**: Increment the BUILD number (fourth part) in CHANGELOG.md for each change
- **Version Alignment**: The first three parts of CHANGELOG.md version should match Cargo.toml version (e.g., CHANGELOG.md: 0.1.2.5 → Cargo.toml: 0.1.2)
- **Chronological Order**: Newest entries at the top, oldest at the bottom
- **Comprehensive Documentation**: Include:
  - **Changes**: What was modified, added, or removed
  - **Validation**: ✅ checkmarks for verified functionality
  - **Technical Details**: Implementation specifics and architectural decisions
  - **Impact**: How changes affect existing functionality
- **Consistent Format**: Follow the established changelog format with proper sections and markdown
- **Date Tracking**: Include date (YYYY-MM-DD) for each version entry

### CLI-Specific Best Practices
- **Argument Parsing**: Implement robust command-line argument parsing
- **Help Systems**: Provide comprehensive help and usage information
- **Exit Codes**: Use appropriate exit codes for different scenarios
- **User Experience**: Design intuitive command syntax and clear output
- **Error Messages**: Provide actionable error messages to users
- **Configuration**: Support configuration files and environment variables

### Rust-Specific Guidelines
- **Ownership**: Understand and properly use ownership, borrowing, and lifetimes
- **Pattern Matching**: Use `match` statements for comprehensive case handling
- **Traits**: Implement and use traits for code reuse and polymorphism
- **Generics**: Use generics appropriately for type flexibility
- **Cargo**: Leverage Cargo for building, testing, and project management
- **Edition**: Use Rust 2024 edition features and best practices
