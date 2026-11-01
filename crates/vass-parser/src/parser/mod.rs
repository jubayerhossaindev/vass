//! Parser module
//!
//! Entry points for parsing `.vass` (indented) and `.vcss` (braces) source.

pub mod base;
pub mod braces;
pub mod indented;
pub mod selector;
pub mod stylesheet;

pub use vass_core::ast::input::Syntax;
pub use vass_core::ast::input::statement::Stylesheet;
pub use vass_core::error::Result;
pub use stylesheet::{parse_expr_snippet, parse_interp_text};

/// Parse `source` using `syntax`
pub fn parse(source: &str, syntax: Syntax) -> Result<Stylesheet> {
  match syntax {
    Syntax::Indented => indented::parse(source),
    Syntax::Braces => braces::parse(source),
  }
}

/// Guess the syntax from a file extension
pub fn detect_syntax(path: &std::path::Path) -> Syntax {
  match path.extension().and_then(|e| e.to_str()) {
    Some("vass") => Syntax::Indented,
    _ => Syntax::Braces,
  }
}
