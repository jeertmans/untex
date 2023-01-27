//! Highlighting parts of LaTeX documents via [`Token`] iterators.
use crate::error::Result;
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
    fn highlight_spans(self) -> FilterMap<Self, fn(Self::Item) -> Option<Span>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (_, span))| if b { Some(span) } else { None })
    }

    /// Returns highlight tokens.
    fn highlight_tokens(self) -> FilterMap<Self, fn(Self::Item) -> Option<Token<'source>>>
    where
        Self: Sized,
    {
        self.filter_map(|(b, (token, _))| if b { Some(token) } else { None })
    }

    /// Returns highlight spanned tokens.
    fn highlight_spanned_tokens(
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
        buffer: &mut W,
        highlight_color: &ColorSpec,
    ) -> Result<()>
    where
        W: WriteColor,
    {
        buffer.reset()?;

        for (is_highlighted, (_, span)) in self {
            if is_highlighted {
                buffer.set_color(highlight_color)?;
                buffer.write_all(source[span].as_bytes())?;
                buffer.reset()?;
            } else {
                buffer.write_all(source[span].as_bytes())?;
            }
        }
        Ok(())
    }
}

impl<'source, I> Highlighter<'source> for I where I: Iterator<Item = (bool, SpannedToken<'source>)> {}

/// Highlights a specific token through its (discriminant) name.
#[cfg(feature = "strum")]
#[derive(Debug)]
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
    /// Create a new token highlighter.
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

/// Highlights tokens within math mode.
#[derive(Debug)]
pub struct MathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    in_math_mode: bool,
    closing_token: Option<Token<'source>>,
}

impl<'source, I> MathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new math mode highlighter.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            in_math_mode: false,
            closing_token: None,
        }
    }
}

impl<'source, I> Iterator for MathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) => {
                if self.in_math_mode {
                    if token == self.closing_token.as_ref().cloned().unwrap() {
                        self.in_math_mode = false;
                        self.closing_token = None;
                    }
                    Some((true, (token, span)))
                } else {
                    self.in_math_mode = true;
                    match token {
                        Token::DisplayMathOpen => {
                            self.closing_token = Some(Token::DisplayMathClose)
                        }
                        Token::DollarSign => self.closing_token = Some(Token::DollarSign),
                        Token::DoubleDollarSign => {
                            self.closing_token = Some(Token::DoubleDollarSign)
                        }
                        Token::EnvironmentBegin(name)
                            if matches!(name, "equation" | "equation*" | "align" | "align*") =>
                        {
                            self.closing_token = Some(Token::EnvironmentEnd(name))
                        }
                        Token::InlineMathOpen => self.closing_token = Some(Token::InlineMathClose),
                        _ => self.in_math_mode = false,
                    }

                    Some((self.in_math_mode, (token, span)))
                }
            }
            None => None,
        }
    }
}

/// Highlights tokens within preamble.
#[derive(Debug)]
pub struct PreambleHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    in_preamble: bool,
}

impl<'source, I> PreambleHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new preamble highlighter.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            in_preamble: false,
        }
    }
}

impl<'source, I> Iterator for PreambleHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) => {
                match token {
                    Token::DocumentClass => self.in_preamble = true,
                    Token::EnvironmentBegin("document") => self.in_preamble = false,
                    _ => (),
                }
                Some((self.in_preamble, (token, span)))
            }
            None => None,
        }
    }
}

/// Highlights tokens within document.
#[derive(Debug)]
pub struct DocumentHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    in_document: bool,
}

impl<'source, I> DocumentHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new preamble highlighter.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            in_document: false,
        }
    }
}

impl<'source, I> Iterator for DocumentHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) => {
                match token {
                    Token::EnvironmentBegin("document") => self.in_document = true,
                    Token::EnvironmentEnd("document") => {
                        self.in_document = false;
                        return Some((true, (token, span)));
                    }
                    _ => (),
                }
                Some((self.in_document, (token, span)))
            }
            None => None,
        }
    }
}

/// Highlights tokens within display math mode.
#[derive(Debug)]
pub struct DisplayMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    in_math_mode: bool,
    closing_token: Option<Token<'source>>,
}

impl<'source, I> DisplayMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new display math mode highlighter.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            in_math_mode: false,
            closing_token: None,
        }
    }
}

impl<'source, I> Iterator for DisplayMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) => {
                if self.in_math_mode {
                    if token == self.closing_token.as_ref().cloned().unwrap() {
                        self.in_math_mode = false;
                        self.closing_token = None;
                    }
                    Some((true, (token, span)))
                } else {
                    self.in_math_mode = true;
                    match token {
                        Token::DisplayMathOpen => {
                            self.closing_token = Some(Token::DisplayMathClose)
                        }
                        Token::DoubleDollarSign => {
                            self.closing_token = Some(Token::DoubleDollarSign)
                        }
                        Token::EnvironmentBegin(name)
                            if matches!(name, "equation" | "equation*" | "align" | "align*") =>
                        {
                            self.closing_token = Some(Token::EnvironmentEnd(name))
                        }
                        _ => self.in_math_mode = false,
                    }

                    Some((self.in_math_mode, (token, span)))
                }
            }
            None => None,
        }
    }
}

/// Highlights tokens within inline math mode.
#[derive(Debug)]
pub struct InlineMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
    in_math_mode: bool,
    closing_token: Option<Token<'source>>,
}

impl<'source, I> InlineMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new inline math mode highlighter.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            in_math_mode: false,
            closing_token: None,
        }
    }
}

impl<'source, I> Iterator for InlineMathHighlighter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = (bool, SpannedToken<'source>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, span)) => {
                if self.in_math_mode {
                    if token == self.closing_token.as_ref().cloned().unwrap() {
                        self.in_math_mode = false;
                        self.closing_token = None;
                    }
                    Some((true, (token, span)))
                } else {
                    self.in_math_mode = true;
                    match token {
                        Token::DollarSign => self.closing_token = Some(Token::DollarSign),
                        Token::InlineMathOpen => self.closing_token = Some(Token::InlineMathClose),
                        _ => self.in_math_mode = false,
                    }

                    Some((self.in_math_mode, (token, span)))
                }
            }
            None => None,
        }
    }
}
