use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

use crate::parser::Parser;

#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn start(parser: &mut Parser) -> Result<usize, ()> {
        if let Some(Spanned(_, span)) = parser.current()? {
            Ok(span.0)
        } else {
            Ok(parser.source.1.len())
        }
    }

    pub fn with_start(parser: &mut Parser, start: usize) -> Result<Self, ()> {
        if let Some(Spanned(_, span)) = parser.current()? {
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

pub struct Spanned<T>(pub T, pub Span);

impl<T> Debug for Spanned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0, &self.1).fmt(f)
    }
}

impl<T> Spanned<T> {
    pub fn span(&self) -> Span {
        self.1
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait WithSpan {
    fn with_span(self, span: Span) -> Spanned<Self>
    where
        Self: Sized;
}

impl<T> WithSpan for T {
    fn with_span(self, span: Span) -> Spanned<Self> {
        Spanned(self, span)
    }
}
