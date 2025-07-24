//! User interface module for the CLI application.
//!
//! This module provides all UI-related functionality including:
//! - Display formatting and terminal output
//! - Input reading and prompt handling
//! - Error display and messaging
//! - Terminal utilities and screen management

pub mod disp;

// Re-export commonly used items
pub use disp::{DisplayManager, TerminalUtils};
