//! (La)TeX code highlighting with [`latex::highlight`](crate::latex::highlight).

use crate::cli::io::{InputArgs, OutputArgs};
use crate::cli::traits::Execute;
use crate::latex::highlight::*;
use crate::latex::token::{Token, TokenDiscriminants};
use clap::{Parser, ValueEnum};
use logos::Logos;

/// Define the part of TeX code to be highlighted.
#[derive(Clone, Debug, ValueEnum)]
enum HighlightedPart {
    Math,
    Preamble,
    Document,
    InlineMath,
    DisplayMath,
}

/*
#[derive(Clone, Debug)]
pub enum Test {
    Part(HighlightedPart),
    Token(TokenDiscriminants),
}


impl ValueEnum for Test {
    fn value_variants<'a>() -> &'a [Self] ⓘ{
        &[]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Part(p) => p.to_possible_value(),
            Self::Token(t) => t.to_possible_value(),
        }
    }
}*/


/// Command structure to highlight parts of TeX codes.
#[derive(Debug, Parser)]
#[command(
    about = "Highlight parts of TeX document(s) in a given color or return span locations.",
    allow_missing_positional(true)
)]
pub struct HighlightCommand {
    /// Part to be highlighted.
    /// Required unless `--token` is present.
    #[arg(required_unless_present("token"), conflicts_with("token"), index(1), value_enum, ignore_case = true)]
    part: Option<HighlightedPart>,
    /// Token to be highlighted.
    #[arg(short, long, value_enum, ignore_case = true)]
    pub token: Option<TokenDiscriminants>,
    //#[arg(short, long, value_enum, ignore_case = true)]
    //pub test: Option<Test>,
    #[command(flatten)]
    pub input_args: InputArgs,
    #[command(flatten)]
    pub output_args: OutputArgs,
}

impl Execute for HighlightCommand {
    type Error = std::io::Error;
    fn execute(self) -> Result<(), Self::Error> {
        let mut stdout = self.output_args.stdout();
        let sources = self.input_args.read_sources().unwrap();

        if self.part.is_some() {
            unimplemented!("Highlighting [PART] is currently not implemented");
        }

        let token = self.token.unwrap();
        let color = self.output_args.color_args.into();

        for source in sources.iter() {
            let iter = Token::lexer(source.as_str()).spanned();

            TokenHighlighter::new(iter, token)
                .write_colorized(source.as_str(), &mut stdout, &color)
                .unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_highlight() {
        HighlightCommand::command().debug_assert();
    }
}
