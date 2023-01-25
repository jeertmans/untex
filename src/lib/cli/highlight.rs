//! (La)TeX code highlighting with [`latex::highlight`](crate::latex::highlight).

use crate::cli::io::{InputArgs, OutputArgs};
use crate::cli::traits::Execute;
use crate::latex::highlight::*;
use crate::latex::token::{Token, TokenDiscriminants};
use clap::{CommandFactory, Parser, ValueEnum};
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

/// Command structure to highlight parts of TeX codes.
#[derive(Debug, Parser)]
#[command(
    about = "Highlight parts of TeX document(s) in a given color or return span locations.",
    arg_required_else_help = true,
    override_usage = "untex highlight [OPTIONS] [PART] [-- <FILENAMES>...]\n
    untex highlight [OPTIONS] [-t|--token] <TOKEN> [-- <FILENAMES>...]\n
    command | untex highlight [OPTIONS] [PART]\n
    command | untex highlight [OPTIONS] [-t|--token] <TOKEN>"
)]
pub struct HighlightCommand {
    /// Part to be highlighted.
    /// Required unless `--token` is present.
    #[arg(
        required_unless_present("token"),
        conflicts_with("token"),
        value_enum,
        ignore_case = true
    )]
    part: Option<HighlightedPart>,
    /// Token to be highlighted.
    #[arg(short, long, value_enum, ignore_case = true)]
    pub token: Option<TokenDiscriminants>,
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
            Self::command().error(clap::error::ErrorKind::InvalidValue, "Highlighting [PART] is currently not implemented, and its development can be followed on https://github.com/jeertmans/untex/pull/8").exit();
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
    use clap::{CommandFactory, Parser};
    #[test]
    fn test_highlight() {
        HighlightCommand::command().debug_assert();
    }
    #[test]
    fn test_part_and_one_file_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "math", "--", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_part_and_two_files_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "math", "--", "README.md", "LICENSE.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
    #[test]
    fn test_missing_part_and_one_file_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "README.md"]);
        assert!(m.is_err());
    }
    #[test]
    fn test_missing_part_and_two_files_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "--", "README.md", "LICENSE.md"]);
        assert!(m.is_err());
    }
    #[test]
    fn test_token_and_one_file_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "--token", "comment", "--", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_token_and_two_files_trailing() {
        let m = HighlightCommand::try_parse_from(vec![
            "",
            "--token",
            "comment",
            "--",
            "README.md",
            "LICENSE.md",
        ]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
}
