use regex::Regex;
use std::str::CharIndices;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Comment,
    Linebreak,
    Command,
    Text,
    Error, // Syntax error
}

/// A proper TeX lever must implement this trait.
pub trait Lexer<'source>: Iterator<Item = Token> {
    fn slice(&self) -> &'source str;
}

pub struct OneTokenLexer<'source> {
    source: &'source str,
    token: Token,
    has_iterated: bool,
}

impl<'source> OneTokenLexer<'source> {
    pub fn new(source: &'source str, token: Token) -> Self {
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
}

impl<'source> Iterator for OneTokenLexer<'source> {
    type Item = Token;

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
    pub lineno: usize,
}

impl<'source> BasicLexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            char_iter: source.char_indices(),
            start: 0,
            last_char: None,
            lineno: 0,
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
}

impl<'source> Iterator for BasicLexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // If not none, current char is last char read
        let current_char = match self.last_char {
            Some(c) => Some(c),
            None => self.char_iter.next(),
        };

        match current_char {
            None => None,
            Some((i, c)) => {
                self.start = i;
                match c {
                    '\n' => {
                        // A linebreak is ended by anything that is not as space, a tabulate or a carriage return
                        self.lineno += 1;

                        loop {
                            self.last_char = self.char_iter.next();

                            if self.last_char.is_none() {
                                break;
                            }

                            match self.last_char.unwrap().1 {
                                ' ' | '\r' | '\t' => continue,
                                _ => break,
                            }
                        }
                        Some(Token::Linebreak)
                    }
                    '%' => {
                        // A comment is ended by a linebreak
                        loop {
                            self.last_char = self.char_iter.next();

                            if self.last_char.is_none() {
                                break;
                            }

                            match self.last_char.unwrap().1 {
                                '\n' => {
                                    self.lineno += 1;
                                    break;
                                }
                                _ => continue,
                            }
                        }
                        Some(Token::Comment)
                    }
                    '\\' => {
                        // A command is quite complicated...

                        self.last_char = self.char_iter.next();

                        match self.last_char {
                            None => Some(Token::Error),
                            Some((_, c)) => match c {
                                'a'..='z' | 'A'..='Z' => {
                                    // First we read the command name
                                    loop {
                                        self.last_char = self.char_iter.next();

                                        match self.last_char {
                                            None => return Some(Token::Command), // It was last character
                                            Some((_, c)) => match c {
                                                'a'..='z' | 'A'..='Z' => continue,
                                                '{' | '[' => break,
                                                _ => return Some(Token::Command), // Anything else after the name ends the command
                                            },
                                        }
                                    }

                                    // Then we look for optional or mandatory arguments
                                    loop {
                                        let brac = self.last_char.unwrap().1;
                                        match brac {
                                            '{' | '[' => {
                                                let mut level = 1; // Used to check if we have nested brackets // braces
                                                loop {
                                                    self.last_char = self.char_iter.next();
                                                    // [ + 2 = ], { + 2 = } in ascii
                                                    let c_brac = ((brac as u8) + 2) as char;
                                                    // So `c_brac` closes `brac`

                                                    match self.last_char {
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
                                                                self.last_char =
                                                                    self.char_iter.next();
                                                                if self.last_char.is_none() {
                                                                    break;
                                                                }
                                                            } else if c == '\n' {
                                                                self.lineno += 1;
                                                            }
                                                        }
                                                    }
                                                }

                                                if level != 0 {
                                                    return Some(Token::Error);
                                                }

                                                self.last_char = self.char_iter.next();

                                                if self.last_char.is_none() {
                                                    break;
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                    Some(Token::Command)
                                }
                                _ => {
                                    // '\' is just used tp escape character
                                    self.last_char = self.char_iter.next();
                                    self.last_char = self.char_iter.next();
                                    Some(Token::Command)
                                }
                            },
                        }
                    }
                    _ => {
                        // A text is ended by any other starting token (Comment, ...)
                        loop {
                            self.last_char = self.char_iter.next();

                            if self.last_char.is_none() {
                                break;
                            }

                            match self.last_char.unwrap().1 {
                                '\n' => {
                                    self.lineno += 1;
                                    break;
                                }
                                '%' | '\\' => break,
                                _ => continue,
                            }
                        }
                        Some(Token::Text)
                    }
                }
            }
        }
    }
}

pub struct RecursiveLexer<'source> {
    lexers: Vec<Box<dyn Lexer<'source> + 'source>>,
    command_re: Vec<Regex>,
}

impl<'source> RecursiveLexer<'source> {
    pub fn new(source: &'source str, command_re: Vec<Regex>) -> Self {
        Self {
            lexers: vec![Box::new(BasicLexer::new(source))],
            command_re,
        }
    }
}

impl<'source> Lexer<'source> for RecursiveLexer<'source> {
    fn slice(&self) -> &'source str {
        self.lexers.last().unwrap().slice()
    }
}

impl<'source> Iterator for RecursiveLexer<'source> {
    type Item = Token;

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

                            self.lexers
                                .push(Box::new(BasicLexer::new(caps.get(2).unwrap().as_str())));

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
        let mut file = File::open("tests/data/minimal.tex").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut lex = BasicLexer::new(&contents);

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
