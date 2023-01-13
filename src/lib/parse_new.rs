use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"\\[\\{}_$&%]")]
    EscapedChar,

    #[token(r"\")]
    Backslash,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("_")]
    Underscore,

    #[token("$")]
    SingleDollar,

    #[token("$$")]
    DoubleDollar,

    #[token("&")]
    And,

    #[regex("%.*")]
    Comment,

    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[regex(r"\\[a-zA-Z][a-zA-Z0-9_$]*")]
    Command,

    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[regex("[ \t]+")]
    Spaces,

    #[error]
    Text,
}

fn main() {
    let file = std::env::args().nth(1).unwrap();

    let content = std::fs::read_to_string(&file).unwrap();

    for (token, span) in Token::lexer(&content).spanned() {
        println!("{:#?}: {:?}", token, &content[span]);
    }
}
