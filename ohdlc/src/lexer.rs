use crate::{
    token::{RawToken, Token},
    util,
};

pub struct Lexer<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(mut cursor) = self.current() else {
            return None;
        };
        cursor = self.skip_comments(cursor)?;

        match cursor.0 {
            i if i.is_alphabetic() => Some(self.lex_ident()),
            _ => unimplemented!(),
        }
    }
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Lexer {
            source: source.as_bytes(),
            cursor: 0,
        }
    }

    #[inline(always)]
    fn current(&self) -> Option<Cursor> {
        if self.cursor >= self.source.len() {
            None
        } else {
            let (c, p) = util::next_utf8_point(self.source, self.cursor);
            Some(Cursor(c, p))
        }
    }

    #[inline(always)]
    fn advance(&mut self, coin: Cursor) {
        self.cursor = coin.1;
    }

    #[inline(always)]
    fn skip_whitespace(&mut self, mut cursor: Cursor) -> Option<Cursor> {
        while cursor.0.is_whitespace() {
            self.advance(cursor);
            cursor = self.current()?;
        }
        Some(cursor)
    }

    #[inline(always)]
    fn skip_comments(&mut self, mut cursor: Cursor) -> Option<Cursor> {
        loop {
            cursor = self.skip_whitespace(cursor)?;
            if let Some(c) = self.is_match("//") {
                self.cursor = c;
                cursor = self.current()?;
                while cursor.0 != '\n' {
                    self.advance(cursor);
                    cursor = self.current()?;
                }
            } else if let Some(c) = self.is_match("/*") {
                self.cursor = c;
                const CLOSE: &str = "*/";
                while let None = self.is_match(CLOSE) {
                    if self.cursor >= self.source.len() {
                        panic!("Uncompleted Multiline Comment");
                    } else {
                        self.cursor += CLOSE.len();
                    }
                }
                cursor = self.current()?;
            } else {
                break;
            }
        }
        Some(cursor)
    }

    fn is_match(&self, pattern: &str) -> Option<usize> {
        for (i, b) in pattern.bytes().enumerate() {
            if self.source[self.cursor + i] != b {
                return None;
            }
        }
        Some(pattern.len())
    }

    fn lex_ident(&mut self) -> Token<'s> {
        let start = self.cursor;
        while let Some(Cursor(c, i)) = self.current() {
            if c.is_alphabetic() || c.is_numeric() || c == '_' {
                self.cursor = i;
            } else {
                break;
            }
        }
        let str = unsafe { std::str::from_utf8_unchecked(&self.source[start..self.cursor]) };
        Token(RawToken::Identifier(str))
    }
}

struct Cursor(char, usize);
