//#![warn(missing_docs)]

use crate::chars::CharStream;
use crate::token::TokenStream;
use std::io;

use std::fs::read_to_string;

pub fn write_file_explanation<W: io::Write>(
    filename: &str,
    mut writer: W,
    verbose: bool,
) -> io::Result<()> {
    let contents =
        read_to_string(&filename).unwrap_or_else(|_| panic!("Could not read {:?}", filename));

    let token_stream: TokenStream = CharStream::new(&contents).into();

    for token in token_stream {
        write!(writer, "{}", token)?;
    }

    Ok(())
}
