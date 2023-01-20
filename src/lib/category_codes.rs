//! Category Codes
//!
//! This module provides tools to work category codes.

use logos::Logos;

#[derive(Debug, Logos)]
/// Category codes, as defined in TeX by Topic (section 2.3).
///
/// > Each of the 256 character codes (0–255) has an associated category code, though not necessarily
/// always the same one. There are 16 categories, numbered 0–15. When scanning the input, TEX thus
/// forms character-code–category-code pairs. The input processor sees only these pairs; from them
/// are formed character tokens, control sequence tokens, and parameter tokens. These tokens are then passed to TEX’s expansion and execution processes.
/// >
/// > A character token is a character-code–category-code pair that is passed unchanged.
/// A control sequence token consists of one or more characters preceded by an escape character;
/// see below. Parameter tokens are also explained below.
///
/// The documentation of each enum variant is simple a copy / paste
/// from aforementionned book.
pub enum CategoryCode {
    /// Escape character; this signals the start of a control sequence.
    ///
    /// IniTEX makes the backslash \ (code 92) an escape character.
    #[token(r"\")]
    EscapedChar = 0,
    /// Beginning of group; such a character causes TEX to enter a new level of grouping.
    ///
    /// The plain format makes the open brace { a beginningof-group character.
    #[token("{")]
    GroupBegin = 1,
    ///  End of group; TEX closes the current level of grouping.
    ///
    ///  Plain TEX has the closing brace } as end-of-group character.
    #[token("}")]
    GroupEnd = 2,
    ///  Math shift; this is the opening and closing delimiter for math formulas.
    ///
    ///  Plain TEX uses the dollar sign $ for this.
    #[token("$")]
    MathShift = 3,
    ///  Alignment tab; the column (row) separator in tables made with \halign (\valign).
    ///
    ///  In plain TEX this is the ampersand &.
    #[token("&")]
    AlignmentTab = 4,
    ///  End of line; a character that TEX considers to signal the end of an input line.
    ///
    ///  IniTEX assigns this code to the hreturni, that is, code 13. Not coincidentally, 13 is also
    ///  the value that IniTEX assigns to the \endlinechar parameter; see above.
    #[token("\n")]
    EndOfLine = 5,
    /// Parameter character; this indicates parameters for macros.
    ///
    /// In plain TEX this is the hash sign #.
    #[token("#")]
    ParameterChar = 6,
    /// Superscript; this precedes superscript expressions in math mode.
    ///
    /// It is also used to denote character codes that cannot be entered in an input file; see below.
    /// In plain TEX this is the circumflex ^.
    #[token("^")]
    Superscript = 7,
    /// Subscript; this precedes subscript expressions in math mode.
    ///
    /// In plain TEX the underscore _ is used for this.
    #[token("_")]
    Subscript = 8,
    /// Ignored; characters of this category are removed from the input, and have therefore
    /// no influence on further TEX processing.
    ///
    /// In plain TEX this is the `null` character, that is, code 0.
    #[token("\x00")]
    Ignored = 9,
    /// Space; space characters receive special treatment.
    ///
    /// IniTEX assigns this category to the ASCII `space` character, code 32.
    #[token(b" ")]
    Space = 10,
    ///  Letter; in IniTEX only the characters `a..z`, `A..Z` are in this category.
    ///
    ///  Often, macropackages make some *'secret'* character (for instance @) into a letter.
    #[regex(r"[a-zA-Z]", priority = 2)]
    Letter = 11,
    /// Other; IniTEX puts everything that is not in the other categories into this category.
    ///
    /// Thus it includes, for instance, digits and punctuation.
    #[error]
    Other = 12,
    ///  Active; active characters function as a TEX command, without being preceded by
    ///  an escape character.
    ///
    ///  In plain TEX this is only the tie character ~, which is defined to produce an
    ///  unbreakable space; see page 187.
    #[token(b"~")]
    Active = 13,
    ///  Comment character; from a comment character onwards, TEX considers the rest of
    ///  an input line to be comment and ignores it.
    ///
    ///  In IniTEX the per cent sign % is made a comment character
    #[token(b"%")]
    CommentChar = 14,
    /// Invalid character; this category is for characters that should not appear in the input.
    ///
    /// IniTEX assigns the ASCII `delete` character, code 127, to this category.
    #[token("\x7F")]
    InvalidChar = 15,
}

macro_rules! impl_try_from {
    ($ty:ty) => {
        impl TryFrom<$ty> for CategoryCode {
            type Error = $ty;
            #[inline]
            fn try_from(code: $ty) -> Result<Self, Self::Error> {
                match code {
                    0 => Ok(CategoryCode::EscapedChar),
                    1 => Ok(CategoryCode::GroupBegin),
                    2 => Ok(CategoryCode::GroupEnd),
                    3 => Ok(CategoryCode::MathShift),
                    4 => Ok(CategoryCode::AlignmentTab),
                    5 => Ok(CategoryCode::EndOfLine),
                    6 => Ok(CategoryCode::ParameterChar),
                    7 => Ok(CategoryCode::Superscript),
                    8 => Ok(CategoryCode::Subscript),
                    9 => Ok(CategoryCode::Ignored),
                    10 => Ok(CategoryCode::Space),
                    11 => Ok(CategoryCode::Letter),
                    12 => Ok(CategoryCode::Other),
                    13 => Ok(CategoryCode::Active),
                    14 => Ok(CategoryCode::CommentChar),
                    15 => Ok(CategoryCode::InvalidChar),
                    x => Err(x),
                }
            }
        }
    };
    ($($ty:ty),+ $(,)?) => {
        $(
            impl_try_from!($ty);
        )*
    }
}

impl_try_from!(u8, u16, u32, u64, usize);

macro_rules! impl_into {
    ($ty:ty) => {
        impl From<CategoryCode> for $ty {
            #[inline]
            fn from(code: CategoryCode) -> Self {
                code as Self
            }
        }
    };
    ($($ty:ty),+ $(,)?) => {
        $(
            impl_into!($ty);
        )*
    };
}

impl_into!(u8, u16, u32, u64, usize);

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
