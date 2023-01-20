//! Category Codes
//!
//! This module provides tools to work category codes.

use logos::Logos;

#[derive(Debug, Logos)]
/// Category codes, as defined in TeX by Topic.
pub enum CategoryCode {
    #[token(r"\")]
    EscapedChar = 0,
    #[token("{")]
    GroupBegin = 1,
    #[token("}")]
    GroupEnd = 2,
    #[token("$")]
    MathShift = 3,
    #[token("&")]
    AlignmentTab = 4,
    #[token("\n")]
    EndOfLine = 5,
    #[token("#")]
    ParameterChar = 6,
    #[token("^")]
    Superscript = 7,
    #[token("_")]
    Subscript = 8,
    #[token("\x00")]
    Ignored = 9,
    #[token(b" ")]
    Space = 10,
    #[regex(r"[a-zA-Z]", priority = 2)]
    Letter = 11,
    #[regex(b".")]
    Other = 12,
    #[token(b"~")]
    Active = 13,
    #[token(b"%")]
    CommentChar = 14,
    #[error]
    #[token("\x7F")]
    InvalidChar = 15,
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    macro_rules! assert_catcode_positions {
        ($source:expr, $token:pat, $($pos:expr),+ $(,)?) => {
            let source = $source.as_bytes();

            let positions: Vec<std::ops::Range<usize>> = vec![$($pos),*];
            let spanned_token: Vec<_> = CategoryCode::lexer(source)
                .spanned()
                .filter(|(token, _)| matches!(token, $token))
                .collect();


            let strs: Vec<_> = CategoryCode::lexer(source)
                .spanned()
                .map(|(token, span)| (token, std::str::from_utf8(&source[span]).unwrap().to_string()))
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
    fn catcode_escaped_char() {
        assert_catcode_positions!(r"Should match \", CategoryCode::EscapedChar, 13..14);
    }

    #[test]
    fn catcode_group_begin() {
        assert_catcode_positions!("Should match {", CategoryCode::GroupBegin, 13..14);
    }

    #[test]
    fn catcode_group_end() {
        assert_catcode_positions!("Should match }", CategoryCode::GroupEnd, 13..14);
    }

    #[test]
    fn catcode_math_shift() {
        assert_catcode_positions!("Should match $", CategoryCode::MathShift, 13..14);
    }

    #[test]
    fn catcode_alignment_tab() {
        assert_catcode_positions!("Should match &", CategoryCode::AlignmentTab, 13..14);
    }

    #[test]
    fn catcode_end_of_line() {
        assert_catcode_positions!("Should match \n", CategoryCode::EndOfLine, 13..14);
    }

    #[test]
    fn catcode_parameter_char() {
        assert_catcode_positions!("Should match #", CategoryCode::ParameterChar, 13..14);
    }

    #[test]
    fn catcode_superscript() {
        assert_catcode_positions!("Should match ^", CategoryCode::Superscript, 13..14);
    }

    #[test]
    fn catcode_subscript() {
        assert_catcode_positions!("Should match _", CategoryCode::Subscript, 13..14);
    }

    #[test]
    fn catcode_ignored() {
        assert_catcode_positions!("Should match \x00", CategoryCode::Ignored, 13..14);
    }

    #[test]
    fn catcode_space() {
        assert_catcode_positions!("Should_match_ ", CategoryCode::Space, 13..14);
    }

    #[test]
    fn catcode_letter() {
        for s in 'A'..'Z' {
            let u = s.to_string();
            let l = s.to_ascii_lowercase().to_string();
            assert_catcode_positions!(&u, CategoryCode::Letter, 0..1);
            assert_catcode_positions!(&l, CategoryCode::Letter, 0..1);
        }
    }

    #[test]
    fn catcode_other() {
        for range in [
            1..10,
            11..32,
            33..35,
            39..65,
            91..92,
            93..94,
            96..97,
            124..125,
        ] {
            for b in range {
                let c = b as u8 as char;
                let s = c.to_string();
                assert_catcode_positions!(&s, CategoryCode::Other, 0..1);
            }
        }
    }

    #[test]
    fn catcode_active() {
        assert_catcode_positions!("Should match ~", CategoryCode::Active, 13..14);
    }

    #[test]
    fn catcode_comment_char() {
        assert_catcode_positions!("Should match %", CategoryCode::CommentChar, 13..14);
    }

    #[test]
    fn catcode_invalid_char() {
        assert_catcode_positions!("Should match \x7F", CategoryCode::InvalidChar, 13..14);
    }
}
