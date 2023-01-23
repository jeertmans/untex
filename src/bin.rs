//use untex::parse::{LaTeXDocument, TryFromTokens};
use clap::{Parser, Subcommand};
use is_terminal::IsTerminal;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use untex::latex::highlight::{HighlightCommand, Highlighter, TokenHighlighter};
use untex::prelude::*;

fn parse_filename(s: &str) -> Result<PathBuf, String> {
    let path_buf: PathBuf = s.parse().unwrap();

    if path_buf.is_file() {
        Ok(path_buf)
    } else {
        Err(format!("Invalid filename: {}", s))
    }
}

fn parse_directory(s: &str) -> Result<PathBuf, String> {
    let path_buf: PathBuf = s.parse().unwrap();

    if path_buf.is_dir() {
        Ok(path_buf)
    } else {
        Err(format!("Invalid directory: {}", s))
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    propagate_version(true),
    args_conflicts_with_subcommands(true)
)]
struct Args {
    /// Filename(s) of TeX document(s) that should be used.
    /// Required unless `--stdin` is present, in which case it will be ignored.
    ///
    /// If multiple filenames are provided, calls to `\input{...}` and `\include{...}` are ignored,
    /// even if `follow-includes` is present.
    #[arg(required_unless_present("stdin"), num_args(1..), global = true, value_parser = parse_filename)]
    filenames: Vec<PathBuf>,

    /// If set, read files from calls to `\input{...}` and `\include{...}`.
    #[arg(short, long, global = true)]
    follow_includes: bool,

    /// If set, read document from standard input.
    #[arg(long, global = true)]
    stdin: bool,

    /// Directoy used for relative paths, if `--stdin` is used.
    #[arg(short, long, value_parser = parse_directory, global = true, default_value = ".")]
    directory: PathBuf,

    #[command(subcommand)]
    command: Command,

    #[arg(
        short,
        long,
        global = true,
        value_name = "WHEN",
        default_value = "auto"
    )]
    color: clap::ColorChoice,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(visible_alias = "deps")]
    Dependencies,
    Expand,
    #[clap(visible_alias = "hl")]
    Highlight(HighlightCommand),
    #[clap(visible_alias = "fmt")]
    Format,
    Parse,
}

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

    let sources: Vec<String> = if args.stdin {
        let mut source = String::new();
        let stdin = std::io::stdin();

        loop {
            match stdin.read_line(&mut source) {
                Ok(n) if n > 0 => (),
                _ => break,
            }
        }
        vec![source]
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
                let mut iter = Token::lexer(source.as_str()).spanned();

                TokenHighlighter::new(iter, token)
                    .write_colorized(source.as_str(), &mut stdout, &color)
                    .unwrap();
            }
        }
        _ => (),
    }
}
