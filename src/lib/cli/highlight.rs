//! (La)TeX code highlighting with [`latex::highlight`](crate::latex::highlight).

use crate::cli::io::{InputArgs, OutputArgs};
use crate::cli::traits::Execute;
use crate::error::Error;
use crate::latex::highlight::*;
use crate::latex::token::{Token, TokenDiscriminants};
use clap::{Parser, ValueEnum};
use logos::Logos;

/// Define the part of TeX code to be highlighted.
#[derive(Clone, Debug, ValueEnum)]
#[allow(missing_docs)]
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
    override_usage = "untex highlight [OPTIONS] [FILENAMES]...\n
    untex highlight [OPTIONS] [-p|--part] <PART> [FILENAMES]...\n
    untex highlight [OPTIONS] [-t|--token] <TOKEN> [FILENAMES]...\n
    command | untex highlight [OPTIONS] [-p|--part] <PART>\n
    command | untex highlight [OPTIONS] [-t|--token] <TOKEN>"
)]
pub struct HighlightCommand {
    /// Part to be highlighted.
    /// Cannot be used with `--token <TOKEN>`.
    #[arg(
        short,
        long,
        conflicts_with("token"),
        value_enum,
        ignore_case = true,
        default_value = "math"
    )]
    part: HighlightedPart,
    /// Token to be highlighted.
    /// Cannot be used with `--part <PART>`.
    #[arg(short, long, conflicts_with("part"), value_enum, ignore_case = true)]
    pub token: Option<TokenDiscriminants>,
    #[command(flatten)]
    #[allow(missing_docs)]
    pub input_args: InputArgs,
    #[command(flatten)]
    #[allow(missing_docs)]
    pub output_args: OutputArgs,
}

impl Execute for HighlightCommand {
    type Error = Error;
    fn execute(self) -> Result<(), Self::Error> {
        let mut stdout = self.output_args.stdout();
        let sources = self.input_args.read_sources().unwrap();

        let color = self.output_args.color_args.into();

        for source in sources.iter() {
            let iter = Token::lexer(source.as_str()).spanned();
            if let Some(token) = self.token {
                TokenHighlighter::new(iter, token)
                    .write_colorized(source.as_str(), &mut stdout, &color)
                    .unwrap();
            } else {
                match self.part {
                    HighlightedPart::Math => MathHighlighter::new(iter)
                        .write_colorized(source.as_str(), &mut stdout, &color)
                        .unwrap(),
                    HighlightedPart::Preamble => PreambleHighlighter::new(iter)
                        .write_colorized(source.as_str(), &mut stdout, &color)
                        .unwrap(),
                    HighlightedPart::Document => DocumentHighlighter::new(iter)
                        .write_colorized(source.as_str(), &mut stdout, &color)
                        .unwrap(),
                    HighlightedPart::InlineMath => InlineMathHighlighter::new(iter)
                        .write_colorized(source.as_str(), &mut stdout, &color)
                        .unwrap(),
                    HighlightedPart::DisplayMath => DisplayMathHighlighter::new(iter)
                        .write_colorized(source.as_str(), &mut stdout, &color)
                        .unwrap(),
                }
            };
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
    fn test_default_and_one_file() {
        let m = HighlightCommand::try_parse_from(vec!["", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_default_and_two_files() {
        let m = HighlightCommand::try_parse_from(vec!["", "README.md", "LICENSE.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
    #[test]
    fn test_part_and_one_file() {
        let m = HighlightCommand::try_parse_from(vec!["", "--part", "math", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_part_and_two_files() {
        let m =
            HighlightCommand::try_parse_from(vec!["", "--part", "math", "README.md", "LICENSE.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
    #[test]
    fn test_token_and_one_file() {
        let m = HighlightCommand::try_parse_from(vec!["", "--token", "comment", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_token_and_two_files() {
        let m = HighlightCommand::try_parse_from(vec![
            "",
            "--token",
            "comment",
            "README.md",
            "LICENSE.md",
        ]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
    #[test]
    fn test_default_and_one_file_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "--", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_default_and_two_files_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "--", "README.md", "LICENSE.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(
            m.unwrap().input_args.filenames_str(),
            vec!["README.md", "LICENSE.md"]
        );
    }
    #[test]
    fn test_part_and_one_file_trailing() {
        let m = HighlightCommand::try_parse_from(vec!["", "--part", "math", "--", "README.md"]);
        assert!(m.is_ok(), "{}", m.unwrap_err());
        assert_eq!(m.unwrap().input_args.filenames_str(), vec!["README.md"]);
    }
    #[test]
    fn test_part_and_two_files_trailing() {
        let m = HighlightCommand::try_parse_from(vec![
            "",
            "--part",
            "math",
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
