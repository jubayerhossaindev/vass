//! `vass lint` is an alias for the existing file checker.

pub use super::check::Args;

/// Run the lint command.
pub async fn run(args: Args) -> anyhow::Result<()> {
  super::check::run(args).await
}
