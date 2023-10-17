use std::ops::Range;

use crate::parser::{Parser, TokenRef};

#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn start(parser: &mut Parser) -> Result<usize, ()> {
        if let Some(TokenRef(_, span)) = parser.current()? {
            Ok(span.0)
        } else {
            Ok(parser.source.1.len())
        }
    }

    pub fn with_start(parser: &mut Parser, start: usize) -> Result<Self, ()> {
        if let Some(TokenRef(_, span)) = parser.current()? {
            Ok(Self(start, span.1))
        } else {
            Ok(Self(start, parser.source.1.len()))
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self(value.start, value.end)
    }
}

impl From<&Range<usize>> for Span {
    fn from(value: &Range<usize>) -> Self {
        Self(value.start, value.end)
    }
}

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        self.0..self.1
    }
}
