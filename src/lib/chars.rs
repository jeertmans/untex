use std::str::CharIndices;

/// A char stream similar to CharIndices but
/// that automatically increments the line count.
#[derive(Debug, Clone)]
pub struct CharStream<'source> {
    pub source: &'source str,
    char_iter: CharIndices<'source>,
    pub lineno: usize,
    pub last_char: Option<(usize, char)>,
}

impl<'source> CharStream<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            char_iter: source.char_indices(),
            lineno: 0,
            last_char: None,
        }
    }
}

impl<'source> Iterator for CharStream<'source> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.last_char = self.char_iter.next();
        match self.last_char {
            Some((_, c)) if c == '\n' => self.lineno += 1,
            _ => (),
        }

        self.last_char
    }
}
