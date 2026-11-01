//! # VASS Linter
//!
//! Linter for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Configurable lint rules
//! - Warning and error levels
//! - Auto-fix support

#![warn(missing_docs)]
#![allow(dead_code)]

/// Crate result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Linter errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Lint rule error
  #[error("Lint error: {0}")]
  LintRule(String),
}
