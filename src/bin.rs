//use untex::parse::{LaTeXDocument, TryFromTokens};
use clap::{Parser, Subcommand, ValueEnum};
use is_terminal::IsTerminal;
use std::path::PathBuf;
use termcolor::{ColorChoice, StandardStream};
use untex::cli::*;
use untex::latex::highlight::{HighlightCommand, Highlighter, TokenHighlighter};
use untex::prelude::*;

pub fn main() {
    let args = Args::parse_from(wild::args());

    let mut choice: ColorChoice = match args.color {
        clap::ColorChoice::Auto => ColorChoice::Auto,
        clap::ColorChoice::Always => ColorChoice::Always,
        clap::ColorChoice::Never => ColorChoice::Never,
    };

    if choice == ColorChoice::Auto && !std::io::stdout().is_terminal() {
        choice = ColorChoice::Never;
    }

    let mut stdout = StandardStream::stdout(choice);

    let sources: Vec<String> = if args.filenames.is_empty() {
        vec![read_from_stdin(&mut stdout).unwrap()]
    } else {
        let sources: Result<Vec<String>, _> = args
            .filenames
            .iter()
            .map(|filename| std::fs::read_to_string(&filename))
            .collect();
        sources.unwrap()
    };

    match args.command {
        Command::Highlight(highlight) => {
            let token = highlight.token.unwrap();
            let color = highlight.color_args.into();

            for source in sources.iter() {
                let iter = Token::lexer(source.as_str()).spanned();

                TokenHighlighter::new(iter, token)
                    .write_colorized(source.as_str(), &mut stdout, &color)
                    .unwrap();
            }
        }
        _ => (),
    }
}
