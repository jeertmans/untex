//! Pretty formatting LaTeX documen via [`Token`] iterators.

use crate::latex::token::{SpannedToken, Token};
use std::io;
use std::iter::Peekable;

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

/// Iterator to auto indent a document
///
/// Format with the following rules:
/// - blank spaces only;
/// - no indentation before \begin{document}
/// - one level of indentation for each nested \begin{...}, the corresponding \end{...} command reduces the indentation level back;
/// - we assume the LaTeX code is correct
#[derive(Debug)]
pub struct AutoIndentFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    iter: Peekable<I>,
    inside_document: bool,
    target_indentation_level: u8,
    is_indented: bool,
    indent_chars: String,
}

impl<'source, I> AutoIndentFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    /// Create a new dummy formatter.
    pub fn new(iter: I) -> Self {
        Self { iter: iter.peekable(), inside_document: false, target_indentation_level: 0, is_indented: false , indent_chars: String::from("  ")}
    }
}

impl<'source, I> Iterator for AutoIndentFormatter<'source, I>
where
    I: Iterator<Item = SpannedToken<'source>>,
{
    type Item = SpannedToken<'source>;


    fn next(&mut self) -> Option<Self::Item> {
        // Auto Indent Formatter

        // Pre indent matching
        match self.iter.peek() {
            Some(&(Token::EnvironmentBegin("document"), _)) => {
                self.inside_document = true;
            },
            Some(&(Token::EnvironmentEnd(_), _)) => {
                // To count an end envirornment only once
                if !self.is_indented {
                    self.target_indentation_level -= 1;
                }
            },
            _ =>{}
        };

        if !self.is_indented && self.inside_document {
            match self.iter.peek() {
                // Remove current indent
                Some(&(Token::TabsOrSpaces, _)) => {
                    return self.iter.next();
                },
                _ => {}
            }

            self.is_indented = true;
            let mut indentation_value: String = "".to_owned();
            for _ in 0..self.target_indentation_level{
                indentation_value.push_str(&self.indent_chars)
            }

            // Cannot use .. to define the range because it is a Full Range
            let custom_indentation: SpannedToken<'source> = (Token::OwnedString(String::from(indentation_value)), 0..1);
            Some(custom_indentation)
        } else {
            // Post indent matching
            match self.iter.peek() {
                Some(&(Token::EnvironmentBegin(_), _)) => {
                    self.target_indentation_level += 1;
                },
                Some(&(Token::Newline, _)) => {
                    self.is_indented = false;
                },
                _next => {
                }
                    ,
            };
            self.iter.next()
        }
    }
}
