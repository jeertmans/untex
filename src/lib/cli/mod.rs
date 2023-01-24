//! Command-line tools.
//!
//! This module is specifically designed to be used by UnTeX's binary target.
//! It contains all the content needed to create UnTeX's command-line interface.
//!
//! Each subcommand of the CLI should be runnable only using its arguments.
//! This is why subcommands derive the [`clap::Parser`] trait.
//! ```

pub mod color;
pub mod io;
pub mod traits;
pub use crate::latex::highlight::cli::*;
use clap::{CommandFactory, Parser, Subcommand};
pub use traits::*;
#[cfg(feature = "cli-complete")]
pub mod complete;

/// Main command-line structure. Contains every subcommand.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "UnTex: TeX files manipulations made easy.",
    propagate_version(true),
    subcommand_required(true),
    verbatim_doc_comment
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Check,
    #[clap(visible_alias = "deps")]
    Dependencies,
    Expand,
    #[clap(visible_alias = "hl")]
    Highlight(HighlightCommand),
    #[clap(visible_alias = "fmt")]
    Format,
    Parse,
    #[cfg(feature = "cli-complete")]
    Complete(complete::CompleteCommand),
}

pub fn build_cli() -> clap::Command {
    Cli::command()
}

#[test]
fn test_cli() {
    build_cli().debug_assert();
}
