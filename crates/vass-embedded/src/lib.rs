//! # VASS Embedded
//!
//! Embedded protocol for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Host-compiler communication
//! - Protocol buffer encoding/decoding
//! - Importer dispatching

#![warn(missing_docs)]
#![allow(dead_code)]

/// Crate result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Embedded protocol errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Protocol error
  #[error("Protocol error: {0}")]
  Protocol(String),

  /// IO error
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
}
