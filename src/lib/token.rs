#![warn(missing_docs)]

use crate::CharStream;

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

///
#[derive(Debug, Clone)]
pub struct TokenStream<'source> {
    char_stream: &'source CharStream<'source>,
    start: usize,
}

impl<'source> TokenStream<'source> {
    pub fn new(char_stream: &'source CharStream<'source>) -> Self {
        Self {
            char_stream,
            start: 0,
        }
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
        //self.char_stream.source[start..end]
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
    fn comment_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Comment)
    }

    #[inline]
    fn command_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Command)
    }

    #[inline]
    fn linebreak_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Linebreak)
    }

    #[inline]
    fn math_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Math)
    }

    #[inline]
    fn text_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Text)
    }

    #[inline]
    fn error_token(&self) -> Token<'source> {
        Token::new(self.current_slice(), TokenKind::Error)
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
                                Some((i, c)) if c == ' ' || c == '\r' || c == '\t' => continue,
                                _ => break,
                            }
                        }
                        Some(self.linebreak_token())
                    }
                    '%' => {
                        // A comment is ended by a linebreak
                        loop {
                            match self.next_char() {
                                Some((i, c)) if c == '\n' => break,
                                _ => continue,
                            }
                        }
                        Some(self.command_token())
                    }
                    '\\' => {
                        // A command is quite complicated...

                        match self.next_char() {
                            None => Some(self.error_token()),
                            Some((_, c)) => match c {
                                'a'..='z' | 'A'..='Z' => {
                                    // First we read the command name
                                    loop {
                                        match self.next_char() {
                                            None => return Some(self.command_token()), // It was last character
                                            Some((_, c)) => match c {
                                                'a'..='z' | 'A'..='Z' => continue,
                                                '{' | '[' => break,
                                                _ => return Some(self.command_token()), // Anything else after the name ends the command
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
                                                    return Some(self.error_token());
                                                }

                                                if self.next_char().is_none() {
                                                    break;
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                    Some(self.command_token())
                                }
                                _ => {
                                    // '\' is just used tp escape character
                                    self.next_char();
                                    self.next_char();
                                    Some(self.command_token())
                                }
                            },
                        }
                    }
                    _ => {
                        // A text is ended by any other starting token (Comment, ...)
                        loop {
                            match self.next_char() {
                                None => break,
                                Some((_, c)) if c == '\n' || c == '%' || c == '\\' => break,
                                _ => continue,
                            }
                        }
                        Some(self.text_token())
                    }
                }
            }
        }
    }
}
