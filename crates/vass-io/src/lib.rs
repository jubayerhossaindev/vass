//! # VASS IO
//!
//! File I/O, importers, and watcher for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Async file operations
//! - Import resolution
//! - File watching
//! - Import caching

#![warn(missing_docs)]
#![allow(dead_code)]

use std::path::Path;

/// Crate result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// IO errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// File not found
  #[error("File not found: {0}")]
  FileNotFound(String),

  /// IO error
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  /// Import error
  #[error("Import error: {0}")]
  Import(String),
}

/// Read a file to a string with a friendlier "not found" error
pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
  let path = path.as_ref();
  std::fs::read_to_string(path).map_err(|e| match e.kind() {
    std::io::ErrorKind::NotFound => Error::FileNotFound(path.display().to_string()),
    _ => Error::Io(e),
  })
}
