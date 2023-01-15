use logos::Logos;
use untex::parse::{LaTeXDocument, TryFromTokens};
use untex::token::Token;

pub fn main() {
    let file = std::env::args().nth(1).unwrap();

    let content = std::fs::read_to_string(&file).unwrap();

    for (token, span) in Token::lexer(&content).spanned() {
        println!("{:#?}: {:?}", token, &content[span]);
    }

    LaTeXDocument::try_from_lexer(Token::lexer(&content));
}
