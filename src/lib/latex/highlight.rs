//! Highlighting parts of LaTeX tokens.
#[cfg(feature = "strum")]
use crate::latex::token::TokenDiscriminants;
use crate::latex::token::{Span, SpannedToken, Token};
use std::iter::FilterMap;
#[cfg(feature = "color")]
use termcolor::{ColorSpec, WriteColor};

/// Trait for highlighting tokens.
///
/// The boolean value is [`true`] when the token should be highlighted,
/// [`false`] otherwise.
///
/// This trait is automatically implemented on iterators that emit
/// `(bool, SpannedToken)` items.
#[allow(clippy::type_complexity)]
pub trait Highlighter<'source>: Iterator<Item = (bool, SpannedToken<'source>)> {
    /// Returns highlight spans.
    fn higlight_spans(self) -> FilterMap<Self, fn(Self::Item) -> Option<Span>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (_, span))| if b { Some(span) } else { None })
    }

    /// Returns highlight tokens.
    fn higlight_tokens(self) -> FilterMap<Self, fn(Self::Item) -> Option<Token<'source>>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (token, _))| if b { Some(token) } else { None })
    }

    /// Returns highlight spanned tokens.
    fn higlight_spanned_tokens(
        self,
    ) -> FilterMap<Self, fn(Self::Item) -> Option<SpannedToken<'source>>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, spanned_token)| if b { Some(spanned_token) } else { None })
    }

    /// Writes tokens, using a specific color for highlighted tokens.
    ///
    /// See [`termcolor::ColorSpec`] for more details.
    #[cfg(feature = "color")]
    fn write_colorized<W>(
        &mut self,
        source: &'source str,
        writer: &mut W,
        highlight_color: &ColorSpec,
    ) -> std::io::Result<()>
    where
        W: WriteColor,
    {
        writer.reset()?;

        for (is_highlighted, (_, span)) in self {
            if is_highlighted {
                writer.set_color(highlight_color)?;
                writer.write_all(source[span].as_bytes())?;
                writer.reset()?;
            } else {
                writer.write_all(source[span].as_bytes())?;
            }
        }
        Ok(())
    }
}

impl<'source, I> Highlighter<'source> for I where I: Iterator<Item = (bool, SpannedToken<'source>)> {}

/// Highlights a specific token through its (discriminant) name.
#[cfg(feature = "strum")]
pub struct TokenHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    token: TokenDiscriminants,
}

#[cfg(feature = "strum")]
impl<'source, I> TokenHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    pub fn new(iter: I, token: TokenDiscriminants) -> Self {
        Self { iter, token }
    }
}

#[cfg(feature = "strum")]
impl<'source, I> Iterator for TokenHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) if TokenDiscriminants::from(&token) == self.token => {
                Some((true, (token, span)))
            }
            Some(spanned_token) => Some((false, spanned_token)),
            None => None,
        }
    }
}

#[cfg(feature = "cli")]
pub(crate) mod cli {
    use super::*;
    use crate::cli::io::{InputArgs, OutputArgs};
    use crate::cli::traits::Execute;
    use clap::{Parser, ValueEnum};
    use logos::Logos;

    #[derive(Clone, Debug, ValueEnum)]
    enum HighlightedPart {
        Math,
        Preamble,
        Document,
        InlineMath,
        DisplayMath,
    }

    /// Highlight parts of TeX document(s) in a given color or return span locations.
    #[derive(Debug, Parser)]
    #[command(allow_missing_positional(true))]
    pub struct HighlightCommand {
        /// Part to be highlighted.
        /// Required unless `--token` is present, in which case it will be ignored.
        #[arg(required_unless_present("token"), value_enum)]
        part: HighlightedPart,
        /// Token to be highlighted.
        #[arg(short, long, value_enum)]
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
}
