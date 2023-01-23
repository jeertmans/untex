use clap::Args;
use termcolor::{Color, ColorSpec};

/// Color specification used with [`termcolor::WriteColor`].
///
/// For valid colors, see [`termcolor::Color`].
#[derive(Args, Debug)]
pub struct ColorArgs {
    /// Set the foreground color.
    #[clap(long, default_value = "red")]
    fg: Option<Color>,
    /// Set the background color.
    #[clap(long)]
    bg: Option<Color>,
    /// Set the text to be bold (Unix only).
    #[clap(long)]
    bold: bool,
    /// Set the text to be dimmed (Unix only).
    #[clap(long)]
    dimmed: bool,
    /// Set the text to be italicized (Unix only).
    #[clap(long)]
    italic: bool,
    /// Set the text to be underlined (Unix only).
    #[clap(long)]
    underline: bool,
    /// Set the text to be strikedthrough (Unix only).
    #[clap(long)]
    strikethrough: bool,
    /// Set the text color to be intense.
    #[clap(long)]
    intense: bool,
}

impl From<ColorArgs> for ColorSpec {
    fn from(args: ColorArgs) -> Self {
        let mut color = ColorSpec::new();
        color
            .set_fg(args.fg)
            .set_bg(args.bg)
            .set_bold(args.bold)
            .set_dimmed(args.dimmed)
            .set_italic(args.italic)
            .set_underline(args.underline)
            .set_strikethrough(args.strikethrough)
            .set_intense(args.intense);

        color
    }
}
