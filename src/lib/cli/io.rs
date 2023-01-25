//! Input and Output command-line tools.

use clap::{Args, ValueEnum};
use is_terminal::IsTerminal;
use std::io;
use std::path::PathBuf;
use termcolor::{ColorChoice, StandardStream};

#[derive(Clone, Debug, ValueEnum)]
#[allow(missing_docs)]
/// Enumerate all possible choices for an "automatic" boolean flag.
pub enum Choice {
    Always,
    Auto,
    Never,
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

#[derive(Args, Debug)]
#[allow(missing_docs)]
pub struct InputArgs {
    /// Filename(s) of TeX document(s) that should be used.
    /// If not present, will read content from standard input.
    ///
    /// If multiple filenames are provided, calls to `\input{...}` and `\include{...}` are ignored,
    /// even if `follow-includes` is present.
    #[arg(num_args(1..), last(true), value_parser = parse_filename)]
    pub filenames: Vec<PathBuf>,

    /// If set, read files from calls to `\input{...}` and `\include{...}`.
    #[arg(short, long, value_name("WHEN"), value_enum, default_value = "auto", default_missing_value = "always", num_args(0..=1), require_equals(true))]
    pub follow_includes: Choice,

    /// Directoy used for relative paths, if standard input is used.
    #[arg(short, long, value_parser = parse_directory, default_value = ".")]
    pub directory: PathBuf,
}

impl InputArgs {
    /// Return filename path as vector of string slices.
    #[must_use]
    pub fn filenames_str(&self) -> Vec<&'_ str> {
        self.filenames.iter().map(|p| p.to_str().unwrap()).collect()
    }
    /// Read one or more sources, either from filenames or frind standard input.
    pub fn read_sources(&self) -> io::Result<Vec<String>> {
        let sources: Vec<String> = if self.filenames.is_empty() {
            let mut source = String::new();
            read_from_stdin(&mut io::stdout(), &mut source)?;
            vec![source]
        } else {
            let sources: Result<Vec<String>, _> =
                self.filenames.iter().map(std::fs::read_to_string).collect();
            sources?
        };
        Ok(sources)
    }
}

/// Read lines from standard input and write to buffer string.
///
/// Standard output is used when waiting for user to input text.
pub fn read_from_stdin<W>(stdout: &mut W, buffer: &mut String) -> io::Result<()>
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
    let stdin = std::io::stdin();

    while stdin.read_line(buffer)? > 0 {}
    Ok(())
}

#[derive(Args, Debug)]
#[allow(missing_docs)]
pub struct OutputArgs {
    /// Specify WHEN to colorize output.
    #[arg(short, long, value_name = "WHEN", default_value = "auto", default_missing_value = "always", num_args(0..=1), require_equals(true))]
    pub color: clap::ColorChoice,
    /// Whether output show, when possible, be written in place.
    #[arg(short, long, value_name("WHEN"), value_enum, default_value = "auto", default_missing_value = "always", num_args(0..=1), require_equals(true))]
    pub inplace: Choice,
    /// How the output result should preferably be formatted.
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Auto)]
    pub output_format: OutputFormat,
    #[command(flatten)]
    #[allow(missing_docs)]
    pub color_args: super::color::ColorArgs,
}

impl OutputArgs {
    /// Return a standard output stream that optionally supports color.
    #[must_use]
    pub fn stdout(&self) -> StandardStream {
        let mut choice: ColorChoice = match self.color {
            clap::ColorChoice::Auto => ColorChoice::Auto,
            clap::ColorChoice::Always => ColorChoice::Always,
            clap::ColorChoice::Never => ColorChoice::Never,
        };

        if choice == ColorChoice::Auto && !io::stdout().is_terminal() {
            choice = ColorChoice::Never;
        }

        StandardStream::stdout(choice)
    }
}

/// Output format used by UnTeX (depends on the command).
///
/// - `auto`: default to `colorized` in terminal, `annotated` otherwise
/// - `colorized`: use color
/// - `annotated`: use annotations
/// - `json`: use a json object representation
#[derive(Clone, Debug, ValueEnum)]
#[allow(missing_docs)]
pub enum OutputFormat {
    Auto,
    Colorized,
    Annotated,
    Json,
}
