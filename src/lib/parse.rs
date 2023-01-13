use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"\\[\\{}_$&%]")]
    EscapedChar,
    #[regex("%.*")]
    Comment,
    #[regex(r"\\[\w]+")]
    Command,
    #[regex(r"\r{0,1}\n")]
    Newline,
    #[error]
    Text,
}
