use logos::Logos;
//use untex::parse::{LaTeXDocument, TryFromTokens};
use untex::token::Token;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
   /// Name of the person to greet
   #[arg(required(true), num_args(1..))]
   filenames: Vec<String>,

   #[arg(long, action = clap::ArgAction::SetTrue)]
   follow_includes: bool,
}


pub fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.filenames[0]).unwrap();

    for (token, span) in Token::lexer(&content).spanned() {
        println!("{:#?}: {:?}", token, &content[span]);
    }

    //LaTeXDocument::try_from_lexer(Token::lexer(&content));
}
