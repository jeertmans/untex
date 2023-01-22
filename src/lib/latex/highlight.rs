//! Highlighting parts of LaTeX tokens.
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
pub trait Highlighter<'source>: Iterator<Item = (bool, SpannedToken<'source>)> {
    /// Returns highlight spans.
    fn higlight_spans(self) -> FilterMap<Self, fn(Self::Item) -> Option<Span>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (span, _))| if b { Some(span) } else { None })
    }

    /// Returns highlight tokens.
    fn higlight_tokens(self) -> FilterMap<Self, fn(Self::Item) -> Option<Token<'source>>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (_, token))| if b { Some(token) } else { None })
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
        self,
        source: &'source str,
        writer: &mut W,
        highlight_color: &ColorSpec,
    ) -> std::io::Result<()>
    where
        W: WriteColor,
    {
        writer.reset();

        for (is_highlighted, (span, _)) in self {
            if is_highlighted {
                writer.set_color(highligh_color)?;
                writer.write_all(source[span].as_bytes())?;
                writer.reset();
            } else {
                writer.write_all(source[span].as_bytes())?;
            }
        }
    }
}

impl<'source, I> Highlighter<'source> for I where I: Iterator<Item = (bool, SpannedToken<'source>)> {}

/// Highlights comments.
pub struct CommentHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
}

impl<'source, I> Iterator for CommentHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((span, Token::Comment)) => Some((true, (span, Token::Comment))),
            Some(spanned_token) => Some((false, spanned_token)),
            None => None,
        }
    }
}
