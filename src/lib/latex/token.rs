//! LaTeX tokens, essential for parsing documents.
//!
//! This module defines [`Token`], an enumeration of all important tokens
//! in a LaTeX document, along with other related structures and functions.
//!
//! Most importantly, [`Token`] derives [`Logos`], which allows to create
//! a very efficient lexer from a string slice.

pub use logos::Span;
use logos::{Lexer, Logos};
#[cfg(feature = "strum")]
use strum_macros::EnumDiscriminants;

/// Callback for [`Token::EnvironmentBegin`] that returns the environment name.
fn parse_environment_begin<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    let slice = lex.slice();
    &slice[7..slice.len() - 1]
}

/// Callback for [`Token::EnvironmentEnd`] that returns the environment name.
fn parse_environment_end<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    let slice = lex.slice();
    &slice[5..slice.len() - 1]
}

#[derive(Logos, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "strum", derive(EnumDiscriminants))]
#[cfg_attr(feature = "cli", strum_discriminants(derive(clap::ValueEnum)))]
/// Enumerates all meaningful tokens that can
/// help parse a LaTeX document.
pub enum Token<'source> {
    #[token("&")]
    And,

    #[token("*")]
    Asterix,

    #[token("@")]
    At,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("]")]
    BracketClose,

    #[token("[")]
    BracketOpen,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[regex(r"\\[a-zA-Z]+")]
    CommandName,

    #[regex("%.*")]
    Comment,

    #[token(r"\]")]
    DisplayMathClose,

    #[token(r"\[")]
    DisplayMathOpen,

    #[token(r"\documentclass")]
    DocumentClass,

    #[token("$")]
    DollarSign,

    #[token(".")]
    Dot,

    #[token(r"\\")]
    DoubleBackslash,

    #[token("$$")]
    DoubleDollarSign,

    #[regex(r"\\begin\{[a-zA-Z]+\*?\}", parse_environment_begin)]
    EnvironmentBegin(&'source str),

    #[regex(r"\\end\{[a-zA-Z]+\*?\}", parse_environment_end)]
    EnvironmentEnd(&'source str),

    #[token("=")]
    EqualSign,

    #[token(r"\{")]
    #[token(r"\}")]
    #[token(r"\_")]
    #[token(r"\$")]
    #[token(r"\&")]
    #[token(r"\%")]
    #[token(r"\#")]
    EscapedChar,

    #[token("!")]
    ExclamationMark,

    #[token("#")]
    Hash,

    #[token("^")]
    Hat,

    #[token("-")]
    Hyphen,

    #[token(r"\)")]
    InlineMathClose,

    #[token(r"\(")]
    InlineMathOpen,

    #[token(r"\,")]
    #[token(r"\:")]
    #[token(r"\;")]
    #[token(r"\!")]
    #[token(r"\ ")]
    InsertSpace,

    #[regex(r"\\[^a-zA-Z]")]
    InvalidCommand,

    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[regex("[0-9]+")]
    Number,

    #[error]
    Other,

    /// Variant to be allocated later by the user.
    OwnedString(String),

    #[token(")")]
    ParenClose,

    #[token("(")]
    ParenOpen,

    #[token("+")]
    PlusSign,

    #[token("?")]
    QuestionMark,

    #[token(";")]
    Semicolon,

    #[regex("[ \t]+")]
    TabsOrSpaces,

    #[token("~")]
    Tilde,

    #[token("_")]
    Underscore,

    #[regex(r"[a-zA-Z]+")]
    Word,
}

#[allow(non_upper_case_globals)]
impl<'source> Token<'source> {
    /// Alias to [`Token::Hyphen`] that should be used in math mode.
    pub const MinusSign: Token<'source> = Token::Hyphen;
}

/// A [`Token`] with its [`Span`].
pub type SpannedToken<'source> = (Token<'source>, Span);

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    macro_rules! assert_token_positions {
        ($source:expr, $token:pat, $($pos:expr),+ $(,)?) => {
            let source = $source;

            let positions: Vec<std::ops::Range<usize>> = vec![$($pos),*];
            let spanned_token: Vec<_> = Token::lexer(source)
                .spanned()
                .filter(|(token, _)| matches!(token, $token))
                .collect();


            let strs: Vec<_> = Token::lexer(source)
                .spanned()
                .map(|(token, span)| (token, source[span].to_string()))
                .collect();

            assert_eq!(
                spanned_token.len(), positions.len(),
                "The number of tokens found did not match the expected number of positions {strs:?}"
            );

            for (pos, (token, span)) in positions.into_iter().zip(spanned_token) {
                assert_eq!(
                    pos,
                    span,
                    "Token {token:#?} was found, but expected at {pos:?}"
                );
            }
        };
    }

    #[test]
    fn token_and() {
        assert_token_positions!(r"Should match &, but not \&", Token::And, 13..14,);
    }

    #[test]
    fn token_asterix() {
        assert_token_positions!(
            r"\section*{title} Doing some math $a * b$",
            Token::Asterix,
            8..9,
            36..37
        );
    }

    #[test]
    fn token_at() {
        assert_token_positions!(r"Should match @, but not \@", Token::At, 13..14,);
    }

    #[test]
    fn token_brace_close() {
        assert_token_positions!(r"Should match }, but not \}", Token::BraceClose, 13..14,);
    }

    #[test]
    fn token_brace_open() {
        assert_token_positions!(r"Should match {, but not \{", Token::BraceOpen, 13..14,);
    }

    #[test]
    fn token_bracket_close() {
        assert_token_positions!(r"Should match ], but not \]", Token::BracketClose, 13..14,);
    }

    #[test]
    fn token_bracket_open() {
        assert_token_positions!(r"Should match [, but not \[", Token::BracketOpen, 13..14,);
    }

    #[test]
    fn token_colon() {
        assert_token_positions!(r"Should match :, but not \:", Token::Colon, 13..14,);
    }

    #[test]
    fn token_comma() {
        assert_token_positions!(r"Should match , but not \,", Token::Comma, 13..14,);
    }

    #[test]
    fn token_command_name() {
        assert_token_positions!(
            r"\sin\cos\text{some text}\alpha1234",
            Token::CommandName,
            0..4,
            4..8,
            8..13,
            24..30,
        );
    }

    #[test]
    fn token_comment() {
        assert_token_positions!(
            r"% this is a comment
            \% this is not a comment",
            Token::Comment,
            0..19,
        );
    }

    #[test]
    fn token_display_math_close() {
        assert_token_positions!(
            r"Should match \], but not ]",
            Token::DisplayMathClose,
            13..15,
        );
    }

    #[test]
    fn token_display_math_open() {
        assert_token_positions!(
            r"Should match \[, but not [",
            Token::DisplayMathOpen,
            13..15,
        );
    }

    #[test]
    fn token_document_class() {
        assert_token_positions!(r"\documentclass{article}", Token::DocumentClass, 0..14,);
    }

    #[test]
    fn token_dollar_sign() {
        assert_token_positions!(r"Should match $, but not $$", Token::DollarSign, 13..14,);
    }

    #[test]
    fn token_dot() {
        assert_token_positions!(r"Should match ., but not \.", Token::Dot, 13..14,);
    }

    #[test]
    fn token_double_backslash() {
        assert_token_positions!(
            r"Should match \\, but not \",
            Token::DoubleBackslash,
            13..15,
        );
    }

    #[test]
    fn token_double_dollar_sign() {
        assert_token_positions!(
            r"Should match $$, but not $",
            Token::DoubleDollarSign,
            13..15,
        );
    }

    #[test]
    fn token_environment_begin() {
        assert_token_positions!(
            r"\begin{equation}",
            Token::EnvironmentBegin("equation"),
            0..16,
        );
    }

    #[test]
    fn token_environment_end() {
        assert_token_positions!(r"\end{equation}", Token::EnvironmentEnd("equation"), 0..14,);
    }

    #[test]
    fn token_equal_sign() {
        assert_token_positions!(r"Should match =, but not \=", Token::EqualSign, 13..14,);
    }

    #[test]
    fn token_escaped_char() {
        for s in ["{", "}", "_", "$", "&", "%", "#"] {
            assert_token_positions!(
                &format!("Should match \\{s}, but not {s}"),
                Token::EscapedChar,
                13..15,
            );
        }
    }

    #[test]
    fn token_hash() {
        assert_token_positions!(r"Should match #, but not \#", Token::Hash, 13..14,);
    }

    #[test]
    fn token_hat() {
        assert_token_positions!(r"Should match ^", Token::Hat, 13..14,);
    }

    #[test]
    fn token_hyphen() {
        assert_token_positions!(r"Should match -, but not \-", Token::Hyphen, 13..14,);
    }

    #[test]
    fn token_inline_math_close() {
        assert_token_positions!(
            r"Should match \), but not )",
            Token::InlineMathClose,
            13..15,
        );
    }

    #[test]
    fn token_inline_math_open() {
        assert_token_positions!(r"Should match \(, but not (", Token::InlineMathOpen, 13..15,);
    }

    #[test]
    fn token_insert_space() {
        for s in [",", ":", ";", "!", " "] {
            assert_token_positions!(
                &format!("Should match \\{s}, but not {s}"),
                Token::InsertSpace,
                13..15,
            );
        }
    }

    #[test]
    fn token_invalid_command() {
        assert_token_positions!(
            r"Should match \+, but not \;",
            Token::InvalidCommand,
            13..15,
        );
    }

    #[test]
    fn token_minus_sign() {
        assert_token_positions!(r"Should match -, but not \-", &Token::MinusSign, 13..14,);
    }

    #[test]
    fn token_newline() {
        assert_token_positions!("Hello\nMy name is\r\nJérome", Token::Newline, 5..6, 16..18,);
    }

    #[test]
    fn token_number() {
        assert_token_positions!("0123.456 789", Token::Number, 0..4, 5..8, 9..12,);
    }

    #[test]
    fn token_other() {
        assert_token_positions!("' ` < >", Token::Other, 0..1, 2..3, 4..5, 6..7,);
    }

    #[test]
    fn token_paren_close() {
        assert_token_positions!(r"Should match ), but not \)", Token::ParenClose, 13..14,);
    }

    #[test]
    fn token_paren_open() {
        assert_token_positions!(r"Should match (, but not \(", Token::ParenOpen, 13..14,);
    }

    #[test]
    fn token_plus_sign() {
        assert_token_positions!(r"Should match +, but not \+", Token::PlusSign, 13..14,);
    }

    #[test]
    fn token_question_mark() {
        assert_token_positions!(r"Should match ?, but not \?", Token::QuestionMark, 13..14,);
    }

    #[test]
    fn token_semicolon() {
        assert_token_positions!(r"Should match ;, but not \;", Token::Semicolon, 13..14,);
    }

    #[test]
    fn token_tabs_or_spaces() {
        assert_token_positions!(
            "Should match \t, but not \\ ",
            Token::TabsOrSpaces,
            6..7,
            12..14,
            15..16,
            19..20,
            23..24,
        );
    }

    #[test]
    fn token_tilde() {
        assert_token_positions!(r"Should match ~", Token::Tilde, 13..14,);
    }

    #[test]
    fn token_underscore() {
        assert_token_positions!(r"Should match _, but not \_", Token::Underscore, 13..14,);
    }

    #[test]
    fn token_word() {
        assert_token_positions!(r"Should match words", Token::Word, 0..6, 7..12, 13..18,);
    }

    #[test]
    fn test_document() {
        let source = r#"
\documentclass{article}
\usepackage{tikz}

\begin{document}
    \begin{tikzpicture}[scale=1.5]
        \draw[thick,fill=gray!60] (0,0) rectangle (1,1);
    \end{tikzpicture}
\end{document}
"#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::DocumentClass));
        assert_eq!(lex.next(), Some(Token::BraceOpen));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "article");
        assert_eq!(lex.next(), Some(Token::BraceClose));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::CommandName));
        assert_eq!(lex.next(), Some(Token::BraceOpen));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "tikz");
        assert_eq!(lex.next(), Some(Token::BraceClose));
        assert_eq!(lex.next(), Some(Token::Newline));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::EnvironmentBegin("document")));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::EnvironmentBegin("tikzpicture")));
        assert_eq!(lex.next(), Some(Token::BracketOpen));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "scale");
        assert_eq!(lex.next(), Some(Token::EqualSign));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "1");
        assert_eq!(lex.next(), Some(Token::Dot));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "5");
        assert_eq!(lex.next(), Some(Token::BracketClose));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::CommandName));
        assert_eq!(lex.next(), Some(Token::BracketOpen));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "thick");
        assert_eq!(lex.next(), Some(Token::Comma));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "fill");
        assert_eq!(lex.next(), Some(Token::EqualSign));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "gray");
        assert_eq!(lex.next(), Some(Token::ExclamationMark));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "60");
        assert_eq!(lex.next(), Some(Token::BracketClose));
        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::ParenOpen));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "0");
        assert_eq!(lex.next(), Some(Token::Comma));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "0");
        assert_eq!(lex.next(), Some(Token::ParenClose));
        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::Word));
        assert_eq!(lex.slice(), "rectangle");
        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::ParenOpen));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "1");
        assert_eq!(lex.next(), Some(Token::Comma));
        assert_eq!(lex.next(), Some(Token::Number));
        assert_eq!(lex.slice(), "1");
        assert_eq!(lex.next(), Some(Token::ParenClose));
        assert_eq!(lex.next(), Some(Token::Semicolon));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::TabsOrSpaces));
        assert_eq!(lex.next(), Some(Token::EnvironmentEnd("tikzpicture")));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::EnvironmentEnd("document")));
        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), None);
    }
}
