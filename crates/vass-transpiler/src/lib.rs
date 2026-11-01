//! # VASS Transpiler
//!
//! CSS output generation for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Expanded and compressed CSS serialization
//! - (Source maps arrive in a later phase)

#![warn(missing_docs)]
#![allow(dead_code)]

pub mod css_writer;
pub mod minifier;
pub mod source_map;

pub use vass_core::error::{Error, Result};
pub use css_writer::{write_compressed, write_expanded};

use vass_core::ast::css::CssStylesheet;

/// Output style selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStyle {
  /// Pretty-printed with indentation
  Expanded,
  /// Minified
  Compressed,
}

/// Render a CSS stylesheet as text
pub fn render(sheet: &CssStylesheet, style: OutputStyle) -> String {
  let out = match style {
    OutputStyle::Expanded => write_expanded(sheet),
    OutputStyle::Compressed => crate::minifier::minify(sheet),
  };
  // Non-ASCII output requires a @charset declaration (reference behavior)
  if out.bytes().any(|b| b >= 0x80) && !out.contains("@charset") {
    return format!("@charset \"UTF-8\";\n{}", out);
  }
  out
}

/// Render a CSS stylesheet in expanded form
pub fn render_expanded(sheet: &CssStylesheet) -> String {
  write_expanded(sheet)
}
