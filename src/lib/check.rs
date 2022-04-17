use crate::chars::CharStream;
use crate::deps::{Dependency, PathUtils, RE_INPUT};
use crate::token::{Token, TokenKind, TokenStream};
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

pub type Result = std::result::Result<(), ()>;

fn check_file_recusirve<'source>(filename: PathBuf, main_dir: &'source Path) -> Result {
    let mut dependencies = Vec::<Dependency>::new();

    let filepath = filename.with_main_dir(main_dir);
    let contents =
        read_to_string(&filepath).unwrap_or_else(|_| panic!("Could not read {:?}", filepath));

    let token_stream: TokenStream = CharStream::new(&contents).into();

    for token in token_stream {
        if token.kind == TokenKind::Command {
            if let Some(caps) = RE_INPUT.captures(token.slice) {
                let dep_filename = PathBuf::from(&caps[1]).with_default_extension("tex");

                check_file_recusirve(dep_filename, main_dir)?;
            }
        } else if token.kind == TokenKind::Error {
            return Err(());
        }
    }

    Ok(())
}

pub fn check_file<W: io::Write>(filename: &str, writer: W, recursive: bool) -> Result {
    if recursive {
        let filename = PathBuf::from(filename);
        let main_dir: PathBuf = filename.parent().unwrap().into();

        check_file_recusirve(filename, &main_dir)
    } else {
        let contents =
            read_to_string(&filename).unwrap_or_else(|_| panic!("Could not read {:?}", filename));
        let token_stream: TokenStream = CharStream::new(&contents).into();

        for token in token_stream {
            if token.kind == TokenKind::Error {
                return Err(());
            }
        }
        Ok(())
    }
}
