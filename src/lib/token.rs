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

    #[token(r"\")]
    Backslash,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[regex("%.*")]
    Comment,

    #[token(r"\[")]
    DisplayMathOpen,

    #[token(r"\]")]
    DisplayMathClose,

    #[token(r"\documentclass")]
    DocumentClass,

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
    fn token_backslash() {
        assert_token_positions!(r"Should match \+, but not \\+", Token::Backslash, 13..14,);
    }

    #[test]
    fn token_brace_open() {
        assert_token_positions!(r"Should match {, but not \{", Token::BraceOpen, 13..14,);
    }

    #[test]
    fn token_brace_close() {
        assert_token_positions!(r"Should match }, but not \}", Token::BraceClose, 13..14,);
    }

    #[test]
    fn token_bracket_open() {
        assert_token_positions!(r"Should match [, but not \[", Token::BracketOpen, 13..14,);
    }

    #[test]
    fn token_bracket_close() {
        assert_token_positions!(r"Should match ], but not \]", Token::BracketClose, 13..14,);
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
    fn token_display_math_open() {
        assert_token_positions!(
            r"Should match \[, but not [",
            Token::DisplayMathOpen,
            13..15,
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
    fn token_document_class() {
        assert_token_positions!(r"\documentclass{article}", Token::DocumentClass, 0..14,);
    }

    #[test]
    fn token_dollar_sign() {
        assert_token_positions!(r"Should match $, but not $$", Token::DollarSign, 13..14,);
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
    fn token_environment_begin() {
        assert_token_positions!(r"\begin{equation}", Token::EnvironmentBegin, 0..6,);
    }

    #[test]
    fn token_environment_end() {
        assert_token_positions!(r"\end{equation}", Token::EnvironmentEnd, 0..4,);
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
    fn token_inline_math_open() {
        assert_token_positions!(r"Should match \(, but not (", Token::InlineMathOpen, 13..15,);
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
    fn token_macro_name() {
        assert_token_positions!(
            r"\sin\cos\text{some text}\alpha1234",
            Token::MacroName,
            0..4,
            4..8,
            8..13,
            24..30,
        );
    }

    #[test]
    fn token_newline() {
        assert_token_positions!("Hello\nMy name is\r\nJérome", Token::Newline, 5..6, 16..18,);
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
        assert_token_positions!(
            r"Should match words, commas or symbols!",
            Token::Word,
            0..6,
            7..12,
            13..18,
            18..19,
            20..26,
            27..29,
            30..37,
            37..38,
        );
    }
}
