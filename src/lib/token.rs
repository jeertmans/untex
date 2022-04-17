#![warn(missing_docs)]

use crate::CharStream;
use ansi_term::{Colour, Style};
use lazy_static::lazy_static;
use std::fmt;

/// Enumerates all the possible atoms that can be found in a TeX file.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// A commended part
    Comment,
    /// A linebreak, optionally followed by any number of tabulates or spaces
    Linebreak,
    /// Anything that could be a command (please use a space after a command to properly end it)
    Command,
    /// Math escaped, either with simple $ $ or double $$ $$ dollar signs
    Math,
    /// Anything else, that is assume to be printed out when the TeX file is compiled into PDF
    Text,
    /// An error occured when parsing the TeX file
    Error, // Syntax error
}

lazy_static! {
    pub static ref text_style: Style = Style::new();
    pub static ref linebreak_style: Style = Style::new().on(Colour::Red);
    pub static ref command_style: Colour = Colour::Blue;
    pub static ref comment_style: Colour = Colour::Green;
    pub static ref error_style: Style = Colour::Red.bold();
    pub static ref math_style: Style = Colour::Green.bold();
}

/// A Token is ... TODO
#[derive(PartialEq, Clone, Debug)]
pub struct Token<'source> {
    pub slice: &'source str,
    pub kind: TokenKind,
}

impl<'source> Token<'source> {
    pub fn new(slice: &'source str, kind: TokenKind) -> Self {
        Self { slice, kind }
    }
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::Comment => write!(f, "{}", comment_style.paint(self.slice)),
            TokenKind::Linebreak => write!(f, "{}", linebreak_style.paint(self.slice)),
            TokenKind::Command => write!(f, "{}", command_style.paint(self.slice)),
            TokenKind::Math => write!(f, "{}", math_style.paint(self.slice)),
            TokenKind::Text => write!(f, "{}", text_style.paint(self.slice)),
            TokenKind::Error => write!(f, "{}", error_style.paint(self.slice)),
        }
    }
}

/// TODO
#[derive(Debug)]
pub struct TokenStream<'source> {
    char_stream: CharStream<'source>,
    start: usize,
    current_token_kind: TokenKind,
}

impl<'source> TokenStream<'source> {
    pub fn new(char_stream: CharStream<'source>) -> Self {
        Self {
            char_stream,
            start: 0,
            current_token_kind: TokenKind::Error,
        }
    }

    #[inline]
    fn lineno(&self) -> usize {
        self.char_stream.lineno
    }

    #[inline]
    fn current_kind(&self) -> TokenKind {
        TokenKind::Command
    }

    #[inline]
    fn last_char(&self) -> Option<(usize, char)> {
        self.char_stream.last_char
    }

    #[inline]
    fn next_char(&mut self) -> Option<(usize, char)> {
        self.char_stream.next()
    }

    #[inline]
    fn current_char(&mut self) -> Option<(usize, char)> {
        if let Some(c) = self.last_char() {
            Some(c)
        } else {
            self.next_char()
        }
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'source str {
        &self.char_stream.source[start..end]
    }

    #[inline]
    fn current_slice(&self) -> &'source str {
        let end = match self.last_char() {
            None => self.char_stream.source.len(),
            Some((i, _)) => i,
        };
        self.slice(self.start, end)
    }

    #[inline]
    fn current_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), self.current_token_kind.clone())
    }
}

impl<'source> From<CharStream<'source>> for TokenStream<'source> {
    fn from(char_stream: CharStream<'source>) -> TokenStream<'source> {
        TokenStream::new(char_stream)
    }
}

impl<'source> Iterator for TokenStream<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_char() {
            None => None,
            Some((i, c)) => {
                self.start = i; // Start index for current Token
                match c {
                    '\n' => {
                        // A linebreak is ended by anything that is not as space, a tabulate or a carriage return
                        loop {
                            match self.next_char() {
                                Some((_, c)) if c == ' ' || c == '\r' || c == '\t' => continue,
                                _ => break,
                            }
                        }
                        self.current_token_kind = TokenKind::Linebreak;
                        Some(self.current_token())
                    }
                    '%' => {
                        // A comment is ended by a linebreak
                        loop {
                            match self.next_char() {
                                Some((_, c)) if c == '\n' => break,
                                None => break,
                                _ => continue,
                            }
                        }
                        self.current_token_kind = TokenKind::Comment;
                        Some(self.current_token())
                    }
                    '\\' => {
                        // A command is quite complicated...
                        self.current_token_kind = TokenKind::Command;

                        match self.next_char() {
                            None => {
                                self.current_token_kind = TokenKind::Error;
                                Some(self.current_token())
                            }
                            Some((_, c)) => match c {
                                'a'..='z' | 'A'..='Z' => {
                                    // First we read the command name
                                    loop {
                                        match self.next_char() {
                                            None => return Some(self.current_token()), // It was last character
                                            Some((_, c)) => match c {
                                                'a'..='z' | 'A'..='Z' => continue,
                                                '{' | '[' => break,
                                                _ => return Some(self.current_token()), // Anything else after the name ends the command
                                            },
                                        }
                                    }

                                    // Then we look for optional or mandatory arguments
                                    loop {
                                        let brac = self.last_char().unwrap().1;
                                        match brac {
                                            '{' | '[' => {
                                                let mut level = 1; // Used to check if we have nested brackets // braces
                                                loop {
                                                    // [ + 2 = ], { + 2 = } in ascii
                                                    let c_brac = ((brac as u8) + 2) as char;
                                                    // So `c_brac` closes `brac`

                                                    match self.next_char() {
                                                        None => break,
                                                        Some((_, c)) => {
                                                            if c == brac {
                                                                level += 1;
                                                            } else if c == c_brac {
                                                                level -= 1;
                                                                if level == 0 {
                                                                    break;
                                                                }
                                                            } else if c == '\\' {
                                                                // In this case, we need to skip
                                                                // '\{' or '\[ or ...
                                                                if self.next_char().is_none() {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }

                                                if level != 0 {
                                                    self.current_token_kind = TokenKind::Error;
                                                    return Some(self.current_token());
                                                }

                                                if self.next_char().is_none() {
                                                    break;
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                    Some(self.current_token())
                                }
                                _ => {
                                    // '\' is just used tp escape character
                                    self.next_char();
                                    self.next_char();
                                    Some(self.current_token())
                                }
                            },
                        }
                    }
                    '$' => {
                        // A math escaped env is either surrounded by one or two dollar signs
                        self.current_token_kind = TokenKind::Math;

                        match self.next_char() {
                            None => {
                                self.current_token_kind = TokenKind::Error;
                                return Some(self.current_token());
                            }
                            Some((_, c)) => {
                                // Lookin for next dollar sign
                                loop {
                                    match self.next_char() {
                                        Some((_, ch)) if ch == '$' => {
                                            self.next_char();
                                            break;
                                        }
                                        None => {
                                            self.current_token_kind = TokenKind::Error;
                                            return Some(self.current_token());
                                        }
                                        _ => continue,
                                    }
                                }

                                // Need double dollars
                                if c == '$' {
                                    match self.current_char() {
                                        Some((_, ch)) if ch == '$' => {
                                            self.next_char();
                                        }
                                        _ => {
                                            self.current_token_kind = TokenKind::Error;
                                            return Some(self.current_token());
                                        }
                                    }
                                }
                            }
                        }
                        Some(self.current_token())
                    }
                    _ => {
                        // A text is ended by any other starting token (Comment, ...)
                        loop {
                            match self.next_char() {
                                None => break,
                                Some((_, c)) if c == '\n' || c == '%' || c == '\\' || c == '$' => {
                                    break
                                }
                                _ => continue,
                            }
                        }
                        self.current_token_kind = TokenKind::Text;
                        Some(self.current_token())
                    }
                }
            }
        }
    }
}
