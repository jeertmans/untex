//! (La)TeX code pretty formatting with [`latex::format`](crate::latex::format).

use crate::cli::io::{InputArgs, OutputArgs};
use crate::cli::traits::Execute;
use crate::error::Error;
use crate::latex::format::*;
use crate::latex::token::Token;
use clap::Parser;
use logos::Logos;

/// Command structure to pretty format TeX documents.
#[derive(Debug, Parser)]
#[command(about = "Pretty format TeX document(s).")]
pub struct FormatCommand {
    #[command(flatten)]
    #[allow(missing_docs)]
    pub input_args: InputArgs,
    #[command(flatten)]
    #[allow(missing_docs)]
    pub output_args: OutputArgs,
}

impl Execute for FormatCommand {
    type Error = Error;
    fn execute(self) -> Result<(), Self::Error> {
        let mut stdout = self.output_args.stdout();
        let sources = self.input_args.read_sources().unwrap();

        for source in sources.iter() {
            let iter = Token::lexer(source.as_str()).spanned();

            DummyFormatter::new(iter).write_formatted(source.as_str(), &mut stdout)?;
        }
        Ok(())
    }
}
