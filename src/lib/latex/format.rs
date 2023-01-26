//! Pretty formatting LaTeX documen via [`Token`] iterators.

use crate::latex::token::{SpannedToken, Token};
use std::io;

/// Trait for formatting tokens.
///
/// Formatting tokens consist of generating an iterator of such tokens and,
/// depending on the desired result, for each token:
/// - removing it;
/// - mutating it into another token;
/// - or merging it into one or more tokens.
///
/// This trait is automatically implemented on iterators that emit
/// `SpannedToken` items.
#[allow(clippy::type_complexity)]
pub trait Formatter<'source>: Iterator<Item = SpannedToken<'source>> {
    /// Writes tokens to buffer.
    fn write_formatted<W>(&mut self, source: &'source str, buffer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        for (token, span) in self {
            match token {
                Token::OwnedString(string) => buffer.write_all(string.as_bytes())?,
                _ => buffer.write_all(source[span].as_bytes())?,
            }
        }
        Ok(())
    }
}

impl<'source, I> Formatter<'source> for I where I: Iterator<Item = SpannedToken<'source>> {}

/// Dommy formatter, should be removed before v4.0 is released.
#[derive(Debug)]
pub struct DummyFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: I,
}

impl<'source, I> DummyFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new dummy formatter.
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'source, I> Iterator for DummyFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = SpannedToken<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        // Dummy formatter skips comments
        match self.iter.next() {
            Some((Token::Comment, _)) => self.next(),
            next => next,
        }
    }
}
