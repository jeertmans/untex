use std::str::CharIndices;

#[derive(Debug, Clone)]
pub struct Lexer<'source> {
    source: &'source str,
    char_iter: CharIndices<'source>,
    start: usize,
    last_char: Option<(usize, char)>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            char_iter: source.char_indices(),
            start: 0,
            last_char: None,
        }
    }

    pub fn slice(&self) -> &'source str {
        let end = match self.last_char {
            Some((i, _)) => i,
            None => self.source.len(), // By default, the slice points to everything
        };
        &self.source[self.start..end]
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment,
    Linebreak,
    Command,
    Text,
    Error, // Syntax error
}

impl<'source> Iterator for Lexer<'source> {
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
                                '\n' => break,
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
                                                _ => return Some(Token::Command), // A space after the name ends the command
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
                                '\n' | '%' | '\\' => break,
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

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn token_lexer() {
        let mut file = File::open("tests/data/minimal.tex").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut lex = Lexer::new(&contents);

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
