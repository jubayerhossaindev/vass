//! Main error types

use crate::token::Span;
use std::fmt;

/// The main error type for VASS
#[derive(Debug, Clone)]
pub struct Error {
  /// Error kind
  pub kind: ErrorKind,
  /// Source location
  pub span: Option<Span>,
  /// Error message (Boxed so `Error` stays under clippy's 128-byte
  /// `result_large_err` threshold — it is returned everywhere)
  pub message: Box<str>,
  /// Source file path
  pub file: Option<String>,
  /// Related notes
  pub notes: Vec<Note>,
  /// Rarely-set rich fields (diagnostic code, help) kept behind a Box so
  /// `Error` stays small — it is returned from nearly every function.
  pub extra: Option<Box<ErrorExtra>>,
}

/// Optional extras on an [`Error`]
#[derive(Debug, Clone, Default)]
pub struct ErrorExtra {
  /// Stable diagnostic identifier override (e.g. `E0001`)
  pub code: Option<String>,
  /// Actionable suggestion rendered as `= help:` in terminal output and
  /// appended to LSP messages.
  pub help: Option<String>,
  /// Machine-readable quick fix surfaced by the LSP as a code action
  /// (lightbulb / "quick fix" in editors).
  pub fix: Option<Box<QuickFix>>,
}

/// An editor-applyable fix for the diagnostic it is attached to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickFix {
  /// Human-readable action title shown in the editor menu
  /// (e.g. "Add `@use 'base';`").
  pub title: String,
  /// What to change
  pub kind: FixKind,
}

/// The edit a [`QuickFix`] applies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixKind {
  /// Replace the text covered by the diagnostic's span with new text.
  Replace(String),
  /// Insert text at a 1-based `line`/`column` (same coordinate space as
  /// [`Span`](crate::token::Span))
  Insert {
    /// 1-based line to insert at
    line: u32,
    /// 1-based column to insert at
    column: u32,
    /// The text to insert (should end with a newline)
    text: String,
  },
}

/// Error kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
  /// Parse error
  Parse,
  /// Syntax error
  Syntax,
  /// Type error
  Type,
  /// Undefined variable
  UndefinedVariable,
  /// Undefined function
  UndefinedFunction,
  /// Undefined mixin
  UndefinedMixin,
  /// Duplicate declaration
  Duplicate,
  /// Import error
  Import,
  /// IO error
  Io,
  /// Configuration error
  Config,
  /// Runtime error
  Runtime,
  /// Internal error (bug)
  Internal,
}

/// A note attached to an error
#[derive(Debug, Clone)]
pub struct Note {
  /// Note message
  pub message: String,
  /// Source location
  pub span: Option<Span>,
  /// Source file path
  pub file: Option<String>,
}

impl ErrorKind {
  /// Stable diagnostic code for this kind (rustc-style `E0xxx`)
  pub fn code(&self) -> &'static str {
    match self {
      ErrorKind::Parse => "E0001",
      ErrorKind::Syntax => "E0001",
      ErrorKind::Type => "E0003",
      ErrorKind::UndefinedVariable => "E0004",
      ErrorKind::UndefinedFunction => "E0005",
      ErrorKind::UndefinedMixin => "E0006",
      ErrorKind::Duplicate => "E0007",
      ErrorKind::Import => "E0008",
      ErrorKind::Io => "E0009",
      ErrorKind::Config => "E0010",
      ErrorKind::Runtime => "E0011",
      ErrorKind::Internal => "E0012",
    }
  }
}

impl Error {
  /// Create a new error
  pub fn new(kind: ErrorKind, message: impl Into<Box<str>>) -> Self {
    Self {
      kind,
      span: None,
      message: message.into(),
      file: None,
      notes: Vec::new(),
      extra: None,
    }
  }

  /// The stable diagnostic identifier: the explicit code when set,
  /// otherwise the kind's default (`E0001`…`E0012`).
  pub fn code_or_default(&self) -> &str {
    match self.extra.as_ref().and_then(|x| x.code.as_deref()) {
      Some(c) => c,
      None => self.kind.code(),
    }
  }

  /// The attached help text, if any
  pub fn help(&self) -> Option<&str> {
    self.extra.as_ref().and_then(|x| x.help.as_deref())
  }

  /// The attached editor quick fix, if any
  pub fn fix(&self) -> Option<&QuickFix> {
    self.extra.as_ref().and_then(|x| x.fix.as_deref())
  }

  /// Attach an editor quick fix (LSP code action)
  pub fn with_fix(mut self, title: impl Into<String>, kind: FixKind) -> Self {
    self.extra.get_or_insert_with(Box::default).fix = Some(Box::new(QuickFix {
      title: title.into(),
      kind,
    }));
    self
  }

  /// Attach a span only when the error doesn't carry one yet — inner
  /// errors (e.g. a parse failure inside an `@use`d module) keep their
  /// own, more precise location.
  pub fn with_span_if_none(mut self, span: Span) -> Self {
    if self.span.is_none() {
      self.span = Some(span);
    }
    self
  }

  /// Attach a file path only when the error doesn't carry one yet.
  pub fn with_file_if_none(mut self, file: impl Into<String>) -> Self {
    if self.file.is_none() {
      self.file = Some(file.into());
    }
    self
  }

  /// Add a note attributed to another file (drives the LSP's
  /// `relatedInformation` so editors can link related locations).
  pub fn with_note_in_file(mut self, message: impl Into<String>, file: impl Into<String>) -> Self {
    self.notes.push(Note {
      message: message.into(),
      span: None,
      file: Some(file.into()),
    });
    self
  }

  /// Attach an explicit diagnostic code
  pub fn with_code(mut self, code: impl Into<String>) -> Self {
    self.extra.get_or_insert_with(Box::default).code = Some(code.into());
    self
  }

  /// Attach an actionable help message
  pub fn with_help(mut self, help: impl Into<String>) -> Self {
    self.extra.get_or_insert_with(Box::default).help = Some(help.into());
    self
  }

  /// Add a span to the error
  pub fn with_span(mut self, span: Span) -> Self {
    self.span = Some(span);
    self
  }

  /// Add a file path to the error
  pub fn with_file(mut self, file: impl Into<String>) -> Self {
    self.file = Some(file.into());
    self
  }

  /// Add a note to the error
  pub fn with_note(mut self, message: impl Into<String>) -> Self {
    self.notes.push(Note {
      message: message.into(),
      span: None,
      file: None,
    });
    self
  }

  /// Add a note with span to the error
  pub fn with_note_and_span(mut self, message: impl Into<String>, span: Span) -> Self {
    self.notes.push(Note {
      message: message.into(),
      span: Some(span),
      file: self.file.clone(),
    });
    self
  }

  /// Create a parse error
  pub fn parse(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Parse, message)
  }

  /// Create a syntax error
  pub fn syntax(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Syntax, message)
  }

  /// Create a type error
  pub fn type_error(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Type, message)
  }

  /// Create an undefined variable error
  pub fn undefined_variable(name: impl Into<String>) -> Self {
    Self::new(
      ErrorKind::UndefinedVariable,
      format!("Undefined variable: ${}", name.into()),
    )
  }

  /// Create an undefined function error
  pub fn undefined_function(name: impl Into<String>) -> Self {
    Self::new(
      ErrorKind::UndefinedFunction,
      format!("Undefined function: {}", name.into()),
    )
  }

  /// Create an undefined mixin error
  pub fn undefined_mixin(name: impl Into<String>) -> Self {
    Self::new(
      ErrorKind::UndefinedMixin,
      format!("Undefined mixin: {}", name.into()),
    )
  }

  /// Create an import error
  pub fn import(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Import, message)
  }

  /// Create an IO error
  pub fn io(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Io, message)
  }

  /// Create an internal error (bug)
  pub fn internal(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Internal, message)
  }

  /// Create a runtime evaluation error
  pub fn runtime(message: impl Into<Box<str>>) -> Self {
    Self::new(ErrorKind::Runtime, message)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)?;

    if let Some(ref file) = self.file {
      write!(f, "\n  --> {}", file)?;
      if let Some(span) = self.span {
        write!(f, ":{}:{}", span.start.line, span.start.column)?;
      }
    }

    for note in &self.notes {
      write!(f, "\n  note: {}", note.message)?;
    }

    Ok(())
  }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::io(err.to_string())
  }
}

impl From<Error> for String {
  fn from(err: Error) -> Self {
    err.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_creation() {
    let err = Error::parse("Unexpected token");
    assert_eq!(err.kind, ErrorKind::Parse);
    assert_eq!(&*err.message, "Unexpected token");
  }

  #[test]
  fn test_error_with_span() {
    let span = Span::new(0, 10, 1, 5);
    let err = Error::parse("Test").with_span(span);
    assert!(err.span.is_some());
  }

  #[test]
  fn test_undefined_variable() {
    let err = Error::undefined_variable("color");
    assert_eq!(&*err.message, "Undefined variable: $color");
  }

  #[test]
  fn test_error_display() {
    let err = Error::parse("Test error").with_file("test.vass");
    let s = err.to_string();
    assert!(s.contains("Test error"));
    assert!(s.contains("test.vass"));
  }
}
