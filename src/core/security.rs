//! Security utilities for safe file operations and input sanitization
//!
//! This module provides security functions to prevent common vulnerabilities
//! such as path traversal attacks and terminal escape code injection.

use crate::core::error::CliError;
use crate::core::CliResult;
use std::path::{Path, PathBuf};

/// Maximum file size allowed for loading (100MB)
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Validate and sanitize a file path to prevent path traversal attacks
///
/// # Arguments
/// * `path_str` - The file path string to validate
///
/// # Returns
/// * `Ok(PathBuf)` - The canonical, validated path
/// * `Err(CliError)` - If the path is invalid or traverses outside working directory
///
/// # Security
/// This function prevents:
/// - Path traversal attacks using ".." components
/// - Access to files outside the current working directory
/// - Invalid or non-existent paths
///
/// # Examples
/// ```
/// use sm_menu::core::security::validate_file_path;
///
/// // Valid path
/// let result = validate_file_path("data.txt");
///
/// // Invalid path (path traversal)
/// let result = validate_file_path("../../../etc/passwd");
/// assert!(result.is_err());
/// ```
pub fn validate_file_path(path_str: &str) -> CliResult<PathBuf> {
    // Check for empty path
    if path_str.trim().is_empty() {
        return Err(CliError::invalid_input("File path cannot be empty"));
    }

    let path = Path::new(path_str);

    // Check for path traversal attempts
    for component in path.components() {
        if component == std::path::Component::ParentDir {
            return Err(CliError::invalid_input(
                "Path traversal not allowed (.. components detected)",
            ));
        }
    }

    // Get current working directory
    let cwd = std::env::current_dir()
        .map_err(|e| CliError::execution_error(&format!("Cannot determine working directory: {}", e)))?;

    // Resolve the full path
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };

    // Canonicalize to resolve any symbolic links and normalize the path
    // Note: This requires the file to exist
    match full_path.canonicalize() {
        Ok(canonical_path) => {
            // Ensure the canonical path is still within the working directory
            if !canonical_path.starts_with(&cwd) {
                return Err(CliError::permission_denied(
                    "Access outside working directory is not allowed",
                ));
            }
            Ok(canonical_path)
        }
        Err(e) => {
            // File doesn't exist - validate the path would be safe if it did exist
            // Check if the parent directory is within cwd
            if let Some(parent) = full_path.parent() {
                if !parent.starts_with(&cwd) {
                    return Err(CliError::permission_denied(
                        "Access outside working directory is not allowed",
                    ));
                }
            }

            // Return the original error (file not found)
            Err(CliError::file_not_found(&format!("{}: {}", path_str, e)))
        }
    }
}

/// Sanitize a string for safe display in the terminal
///
/// Removes control characters (except newline and tab) that could
/// be used for terminal escape code injection attacks.
///
/// # Arguments
/// * `s` - The string to sanitize
///
/// # Returns
/// A new string with control characters filtered out
///
/// # Examples
/// ```
/// use sm_menu::core::security::sanitize_for_display;
///
/// let safe = sanitize_for_display("Hello\x1b[31mWorld");
/// assert!(!safe.contains("\x1b"));
/// ```
pub fn sanitize_for_display(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

/// Check if a file size is within acceptable limits
///
/// # Arguments
/// * `size` - The file size in bytes
///
/// # Returns
/// * `Ok(())` - If the file size is acceptable
/// * `Err(CliError)` - If the file is too large
///
/// # Examples
/// ```
/// use sm_menu::core::security::validate_file_size;
///
/// assert!(validate_file_size(1024).is_ok());
/// assert!(validate_file_size(200 * 1024 * 1024).is_err());
/// ```
pub fn validate_file_size(size: u64) -> CliResult<()> {
    if size > MAX_FILE_SIZE {
        Err(CliError::execution_error(&format!(
            "File too large: {} bytes (maximum: {} bytes)",
            size, MAX_FILE_SIZE
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_for_display() {
        // Normal text should pass through
        assert_eq!(sanitize_for_display("Hello World"), "Hello World");

        // Newlines and tabs should be preserved
        assert_eq!(sanitize_for_display("Hello\nWorld"), "Hello\nWorld");
        assert_eq!(sanitize_for_display("Hello\tWorld"), "Hello\tWorld");

        // Control characters should be removed
        let input = "Hello\x1b[31mWorld\x1b[0m";
        let output = sanitize_for_display(input);
        assert!(!output.contains('\x1b'));
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_validate_file_size() {
        // Small files should be fine
        assert!(validate_file_size(0).is_ok());
        assert!(validate_file_size(1024).is_ok());
        assert!(validate_file_size(1024 * 1024).is_ok());

        // Files at or under the limit should be ok
        assert!(validate_file_size(MAX_FILE_SIZE).is_ok());

        // Files over the limit should error
        assert!(validate_file_size(MAX_FILE_SIZE + 1).is_err());
        assert!(validate_file_size(200 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_validate_file_path_empty() {
        assert!(validate_file_path("").is_err());
        assert!(validate_file_path("   ").is_err());
    }

    #[test]
    fn test_validate_file_path_traversal() {
        // Path traversal attempts should fail
        assert!(validate_file_path("../file.txt").is_err());
        assert!(validate_file_path("../../etc/passwd").is_err());
        assert!(validate_file_path("subdir/../../../file.txt").is_err());
    }
}
