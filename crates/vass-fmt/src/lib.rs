//! # VASS Formatter
//!
//! Formatter for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Consistent code formatting
//! - Configurable style options
//! - Both .vass and .vcss support

#![warn(missing_docs)]
#![allow(dead_code)]

/// Crate result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Formatter errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Format error
  #[error("Format error: {0}")]
  Format(String),

  /// Parse error
  #[error("{0}")]
  Parse(String),
}
