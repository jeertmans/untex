#![warn(missing_docs)]

use crate::lexer::{Lexer, RecursiveLexer};
use crate::token::{Token, TokenType};
use ansi_term::{Colour, Style};
use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn explain_file(filename: &str, verbose: bool) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lex = RecursiveLexer::new(
        &contents,
        Some(filename),
        vec![
            Regex::new(r"(^\\title\{)([\s\S]*)(\}$)").unwrap(),
            Regex::new(r"(^\\section\{)([\s\S]*)(\}$)").unwrap(),
        ],
    );

    let text_style = Style::new();
    let linebreak_style = Style::new().on(Colour::Red);
    let command_style = Colour::Blue;
    let comment_style = Colour::Green;
    let error_style = Colour::Red.bold();

    if verbose {
        println!(" ==================");
        println!("| {}      |", text_style.paint("Text colour"));
        println!("| {} |", linebreak_style.paint("Linebreak colour"));
        println!("| {}   |", command_style.paint("Command colour"));
        println!("| {}   |", comment_style.paint("Comment colour"));
        println!("| {}     |", error_style.paint("Error colour"));
        println!(" ==================");
    }

    loop {
        let next = lex.next();

        match next.ttype {
            Some(TokenType::Text) => {
                print!("{}", text_style.paint(lex.slice()));
            }
            Some(TokenType::Linebreak) => {
                print!("{}", linebreak_style.paint(lex.slice()));
            }
            Some(TokenType::Command) => {
                print!("{}", command_style.paint(lex.slice()));
            }
            Some(TokenType::Comment) => {
                print!("{}", comment_style.paint(lex.slice()));
            }
            Some(TokenType::Error) => {
                print!("{}", error_style.paint(lex.slice()));
            }
            Some(TokenType::Math) => break,
            None => break,
        }
    }
}
