#![warn(missing_docs)]

use crate::token::Token;
use regex::Regex;
use std::str::CharIndices;

/// A proper TeX lever must implement this trait.
pub trait Lexer<'source>: Iterator<Item = Token<'source>> {
    /// Returns the slice of the current token.
    fn slice(&self) -> &'source str;

    fn lineno(&self) -> usize;

    fn filename(&self) -> Option<&'source str>;

    fn slice_info(&self) -> Option<String> {
        match self.filename() {
            Some(filename) => Some(format!("{}:{}", filename, self.lineno())),
            None => None,
        }
    }
}

/// A one token lexer that is intented to contain only one token.
pub struct OneTokenLexer<'source> {
    source: &'source str,
    token: Token<'source>,
    has_iterated: bool,
}

impl<'source> OneTokenLexer<'source> {
    /// Creates a new one token lexer from source string and token.
    pub fn new(source: &'source str, token: Token<'source>) -> Self {
        Self {
            source,
            token,
            has_iterated: false,
        }
    }
}

impl<'source> Lexer<'source> for OneTokenLexer<'source> {
    fn slice(&self) -> &'source str {
        self.source
    }

    fn lineno(&self) -> usize {
        0
    }

    fn filename(&self) -> Option<&'source str> {
        None
    }
}

impl<'source> Iterator for OneTokenLexer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_iterated {
            None
        } else {
            self.has_iterated = true;
            Some(self.token.clone())
        }
    }
}

pub struct BasicLexer<'source> {
    source: &'source str,
    char_iter: CharIndices<'source>,
    start: usize,
    last_char: Option<(usize, char)>,
    lineno: usize,
    filename: Option<&'source str>,
}

impl<'source> BasicLexer<'source> {
    pub fn new(source: &'source str, filename: Option<&'source str>) -> Self {
        Self {
            source,
            char_iter: source.char_indices(),
            start: 0,
            last_char: None,
            lineno: 0,
            filename,
        }
    }
}

impl<'source> Lexer<'source> for BasicLexer<'source> {
    fn slice(&self) -> &'source str {
        let end = match self.last_char {
            Some((i, _)) => i,
            None => self.source.len(), // By default, the slice points to everything
        };
        &self.source[self.start..end]
    }

    fn lineno(&self) -> usize {
        self.lineno
    }

    fn filename(&self) -> Option<&'source str> {
        self.filename
    }
}

impl<'source> Iterator for BasicLexer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next()
    }
}

pub struct RecursiveLexer<'source> {
    lexers: Vec<Box<dyn Lexer<'source> + 'source>>,
    command_re: Vec<Regex>,
}

impl<'source> RecursiveLexer<'source> {
    pub fn new(
        source: &'source str,
        filename: Option<&'source str>,
        command_re: Vec<Regex>,
    ) -> Self {
        Self {
            lexers: vec![Box::new(BasicLexer::new(source, filename))],
            command_re,
        }
    }
}

impl<'source> Lexer<'source> for RecursiveLexer<'source> {
    fn slice(&self) -> &'source str {
        self.lexers.last().unwrap().slice()
    }

    fn lineno(&self) -> usize {
        self.lexers.last().unwrap().lineno()
    }

    fn filename(&self) -> Option<&'source str> {
        self.lexers.last().unwrap().filename()
    }
}

impl<'source> Iterator for RecursiveLexer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let n_lexers = self.lexers.len();
        let (next_token, next_slice) = {
            let lexer = self.lexers.last_mut().unwrap();

            (lexer.next(), lexer.slice())
        };

        if n_lexers == 1 && next_token.is_none() {
            return None;
        }

        match next_token {
            Some(Token::Command) => {
                for re in self.command_re.iter() {
                    match re.captures(next_slice) {
                        None => continue,
                        Some(caps) => {
                            //let new_slice: &'source str = &caps[2];
                            self.lexers.push(Box::new(OneTokenLexer::new(
                                caps.get(3).unwrap().as_str(),
                                Token::Command,
                            )));

                            self.lexers.push(Box::new(BasicLexer::new(
                                caps.get(2).unwrap().as_str(),
                                None,
                            )));

                            self.lexers.push(Box::new(OneTokenLexer::new(
                                caps.get(1).unwrap().as_str(),
                                Token::Command,
                            )));

                            return self.next();
                        }
                    }
                }

                Some(Token::Command)
            }
            None => {
                self.lexers.pop();
                self.next()
            }
            Some(token) => Some(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{BasicLexer, Lexer, Token};
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn token_lexer() {
        let filename = "tests/data/minimal.tex";
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut lex = BasicLexer::new(&contents, Some(filename));

        assert_eq!(lex.next(), Some(Token::Command));
        assert_eq!(lex.slice(), r"\documentclass{article}");

        assert_eq!(lex.next(), Some(Token::Linebreak));

        assert_eq!(lex.next(), Some(Token::Command));
        assert_eq!(lex.slice(), r"\usepackage[utf8]{inputenc}");

        assert_eq!(lex.next(), Some(Token::Linebreak));

        assert_eq!(lex.next(), Some(Token::Linebreak));

        assert_eq!(lex.next(), Some(Token::Command));
        assert_eq!(lex.slice(), r"\title{minimal}");

        assert_eq!(lex.next(), Some(Token::Linebreak));
    }
}
