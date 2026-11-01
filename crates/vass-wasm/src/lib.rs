//! # VASS WASM
//!
//! WASM bindings for the VASS CSS preprocessor.
//!
//! ## Features
//!
//! - Browser-based compilation
//! - JavaScript interop
//! - Playground support

#![warn(missing_docs)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

/// Compile VASS source to CSS
#[wasm_bindgen]
pub fn compile(source: &str) -> Result<String, JsValue> {
  // Placeholder - will be implemented in later phases
  Ok(format!("/* Compiled from VASS */\n{}", source))
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
  // Set up panic hook for better error messages
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}
