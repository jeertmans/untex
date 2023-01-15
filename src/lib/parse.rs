use crate::error::Result;
use crate::token::Token;
use logos::Lexer;
use std::iter::Peekable;

type Span = std::ops::Range<usize>;

pub trait TryFromTokens {
    fn try_from_tokens<'source, I>(source: &'source str, iter: &mut Peekable<I>) -> Result<Self>
    where
        Self: Sized,
        I: Iterator<Item = (Token, Span)>;

    fn try_from_lexer<'source>(lexer: Lexer<'source, Token>) -> Result<Self>
    where
        Self: Sized,
    {
        let source = lexer.source();
        let mut iter = lexer.spanned().peekable();
        <Self as TryFromTokens>::try_from_tokens(source, &mut iter)
    }
}

pub struct LaTeXDocument {
    preamble: Preamble,
    document: Document,
}

impl TryFromTokens for LaTeXDocument {
    fn try_from_tokens<'source, I>(source: &'source str, iter: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = (Token, Span)>,
    {
        let preamble = Preamble::try_from_tokens(source, iter)?;
        let document = Document::try_from_tokens(source, iter)?;

        Ok(Self { preamble, document })
    }
}

struct Preamble {}

impl TryFromTokens for Preamble {
    fn try_from_tokens<'source, I>(source: &'source str, iter: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = (Token, Span)>,
    {
        match iter
            .skip_while(|(token, _)| {
                matches!(token, Token::Comment | Token::Newline | Token::TabsOrSpaces)
            })
            .next()
            .expect("Preamble should start with a \\documentclass, but nothing was found")
        {
            (Token::DocumentClass, _) => (),
            (token, span) => panic!(
                "Preamble should start with a \\documentclass, not with {:#?}: {}",
                token, &source[span]
            ),
        }

        while let Some((token, _span)) = iter.peek() {
            match token {
                Token::DocumentBegin => return Ok(Self {}),
                _ => (),
            }
            iter.next();
        }
        Ok(Self {})
    }
}


struct Document {}

impl TryFromTokens for Document {
    fn try_from_tokens<'source, I>(source: &'source str, iter: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = (Token, Span)>,
    {
        let mut iter = iter.skip_while(|(token, _)| {
            matches!(token, Token::Comment | Token::Newline | Token::TabsOrSpaces)
        });

        match iter
            .next()
            .expect("Document should start with a \\begin{{document}}, but nothing was found")
        {
            (Token::DocumentBegin, _) => (),
            (token, span) => panic!(
                "Document should start with a \\begin{{document}}, not with {:#?}: {}",
                token, &source[span]
            ),
        }

        while let Some((token, span)) = iter.next() {
            match token {
                Token::DocumentEnd => {
                    if let Some((token, span)) = iter
                        .skip_while(|(token, _)| {
                            matches!(token, Token::Comment | Token::Newline | Token::TabsOrSpaces)
                        })
                        .next()
                    {
                        panic!(
                            "Unexpected token found after \\end{{document}}: {}",
                            &source[span]
                        );
                    }
                    return Ok(Self {});
                }
                _ => (),
            }
        }
        panic!("Missing \\end{{document}}");
        Ok(Self {})
    }
}

struct Options<'source> {

}

struct Arguments<'source> {

}

struct Environment<'source> {
    name: &'source str,
    opts: Options<'source>,
    args: Arguments<'source>,
    span: Span,
}
