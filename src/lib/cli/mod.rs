pub mod color;
use crate::latex::highlight::HighlightCommand;
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use is_terminal::IsTerminal;
use std::io;
use std::path::PathBuf;
#[cfg(feature = "cli-complete")]
pub mod complete;

/// Read lines from standard input.
pub fn read_from_stdin<W>(stdout: &mut W) -> io::Result<String>
where
    W: io::Write,
{
    if io::stdin().is_terminal() {
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
    Ok(source)
}

/// Output format used by UnTeX (depends on the command).
///
/// - `auto`: default to `colorized` in terminal, `annotated` otherwise
/// - `colorized`: use color
/// - `annotated`: use annotations
/// - `json`: use a json object representation
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
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
pub struct Args {
    /// Filename(s) of TeX document(s) that should be used.
    /// If not present, will read content from standard input.
    ///
    /// If multiple filenames are provided, calls to `\input{...}` and `\include{...}` are ignored,
    /// even if `follow-includes` is present.
    #[arg(num_args(1..), global = true, value_parser = parse_filename)]
    pub filenames: Vec<PathBuf>,

    /// If set, read files from calls to `\input{...}` and `\include{...}`.
    #[arg(short, long, global = true)]
    pub follow_includes: bool,

    /// Directoy used for relative paths, if `--stdin` is used.
    #[arg(short, long, value_parser = parse_directory, global = true, default_value = ".")]
    pub directory: PathBuf,

    #[command(subcommand)]
    pub command: Command,

    #[arg(
        short,
        long,
        global = true,
        value_name = "WHEN",
        default_value = "auto"
    )]
    pub color: clap::ColorChoice,

    #[arg(
        short,
        long,
        global = true,
        value_name = "FORMAT",
        default_value = "auto"
    )]
    pub output_format: OutputFormat,
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
    Args::command()
}

#[test]
fn test_cli() {
    build_cli().debug_assert();
}
