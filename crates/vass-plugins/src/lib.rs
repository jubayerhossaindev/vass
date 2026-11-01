//! # VASS Plugins
//!
//! Plugin system for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Custom function plugins
//! - Custom importers
//! - Dart VASS compatibility adapter
//! - WASM plugin support

#![warn(missing_docs)]
#![allow(dead_code)]

/// Crate result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Plugin errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Plugin not found
  #[error("Plugin not found: {0}")]
  NotFound(String),

  /// Plugin load error
  #[error("Failed to load plugin: {0}")]
  LoadError(String),

  /// Plugin execution error
  #[error("Plugin execution error: {0}")]
  ExecutionError(String),
}
