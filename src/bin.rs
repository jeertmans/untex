//use untex::parse::{LaTeXDocument, TryFromTokens};
use clap::{Parser, Subcommand, ValueEnum};
use is_terminal::IsTerminal;
use std::path::PathBuf;
use termcolor::{ColorChoice, StandardStream};
use untex::latex::highlight::{HighlightCommand, Highlighter, TokenHighlighter};
use untex::prelude::*;

/// Output format used by UnTeX (depends on the command).
///
/// - `auto`: default to `colorized` in terminal, `annotated` otherwise
/// - `colorized`: use color
/// - `annotated`: use annotations
/// - `json`: use a json object representation
#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Auto,
    Colorized,
    Annotated,
    Json,
}

/// Parse a string slice into a [`PathBuf`], and error if the file does not exist.
fn parse_filename(s: &str) -> Result<PathBuf, String> {
    let path_buf: PathBuf = s.parse().unwrap();

    if path_buf.is_file() {
        Ok(path_buf)
    } else {
        Err(format!("Invalid filename: {}", s))
    }
}

/// Parse a string slice into a [`PathBuf`], and error if the directory does not exist.
fn parse_directory(s: &str) -> Result<PathBuf, String> {
    let path_buf: PathBuf = s.parse().unwrap();

    if path_buf.is_dir() {
        Ok(path_buf)
    } else {
        Err(format!("Invalid directory: {}", s))
    }
}

/// UnTeX is a tool for manipulating TeX files.
///
/// Among others, it provides command for:
/// - pretty formatting;
/// - parsing content;
/// - expanding macros;
/// - identifying dependencies;
/// - or highlighting parts of a document, such as comments.
///
/// UnTeX can either read from file(s) or from standard input,
/// and can output its results in multiple formats:
/// - colored text in terminal;
/// - annotated text;
/// - or JSON.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    propagate_version(true),
    args_conflicts_with_subcommands(true),
    verbatim_doc_comment
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

    #[arg(
        short,
        long,
        global = true,
        value_name = "FORMAT",
        default_value = "auto"
    )]
    output_format: OutputFormat,
}

#[derive(Subcommand, Debug)]
enum Command {
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
    Complete,
}

fn read_from_stdin() -> String {
    if std::io::stdin().is_terminal() {
        #[cfg(windows)]
        writeln!(
            stdout,
            "Reading from STDIN, press [CTRL+Z] when you're done."
        )?;

        #[cfg(unix)]
        writeln!(
            stdout,
            "Reading from STDIN, press [CTRL+D] when you're done."
        )?;
    }
    let mut source = String::new();
    let stdin = std::io::stdin();

    loop {
        match stdin.read_line(&mut source) {
            Ok(n) if n > 0 => (),
            _ => break,
        }
    }
    source
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
        vec![read_from_stdin()]
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
