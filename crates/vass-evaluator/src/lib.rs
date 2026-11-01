//! # VASS Evaluator
//!
//! Expression evaluator and built-in functions for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Expression evaluation
//! - Variable scoping
//! - Nested rule flattening with `&` support
//! - Block at-rule handling (`@media`, ...)

#![warn(missing_docs)]
#![allow(dead_code)]

pub mod builtins;
pub mod environment;
pub mod visitor;

pub use vass_core::error::{Error, Result};
pub use visitor::evaluate::{EvalOptions, evaluate, evaluate_with};

/// Evaluate a parsed stylesheet into CSS
pub fn evaluate_stylesheet(
  sheet: &vass_core::ast::input::statement::Stylesheet,
) -> Result<vass_core::ast::css::CssStylesheet> {
  evaluate(sheet)
}
