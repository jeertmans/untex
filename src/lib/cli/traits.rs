//! CLI-related traits.
use clap::Parser;

/// Trait for command that can be executed.
pub trait Execute: Parser {
    /// Error type that can be returned from execution.
    type Error;
    /// Execute the command once and consume self.
    fn execute(self) -> Result<(), Self::Error>;
}
