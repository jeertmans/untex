use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
/// Enumerates all meaningful tokens that can
/// help parse a TeX document.
pub enum Token {
    #[token("&")]
    And,

    #[token("*")]
    Asterix,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(r"\")]
    Backslash,

    #[regex("%.*")]
    Comment,

    #[token(r"\[")]
    DisplayMathOpen,

    #[token(r"\]")]
    DisplayMathClose,

    #[token(r"\begin{document}")]
    DocumentBegin,

    #[token(r"\documentclass")]
    DocumentClass,

    #[token(r"\end{document}")]
    DocumentEnd,

    #[token("$")]
    DollarSign,

    #[token(r"\\")]
    DoubleBackslash,

    #[token("$$")]
    DoubleDollarSign,

    #[token(r"\{")]
    #[token(r"\}")]
    #[token(r"\_")]
    #[token(r"\$")]
    #[token(r"\&")]
    #[token(r"\%")]
    #[token(r"\#")]
    EscapedChar,

    #[token(r"\begin")]
    EnvironmentBegin,

    #[token(r"\end")]
    EnvironmentEnd,

    #[token("#")]
    Hash,

    #[token("^")]
    Hat,

    #[token(r"\(")]
    InlineMathOpen,

    #[token(r"\)")]
    InlineMathClose,

    #[token(r"\,")]
    #[token(r"\:")]
    #[token(r"\;")]
    #[token(r"\!")]
    #[token(r"\ ")]
    InsertSpace,

    #[regex(r"\\[a-zA-Z]+")]
    MacroName,

    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[regex("[ \t]+")]
    TabsOrSpaces,

    #[token("~")]
    Tilde,

    #[token("_")]
    Underscore,

    #[error]
    #[regex(r"\w+")]
    Word,
}
