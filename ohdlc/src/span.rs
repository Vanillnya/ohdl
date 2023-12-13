use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

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

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T>(pub T, pub Span);

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
    fn with_span(self, span: impl Into<Span>) -> Spanned<Self>
    where
        Self: Sized;
}

impl<T> WithSpan for T {
    fn with_span(self, span: impl Into<Span>) -> Spanned<Self> {
        Spanned(self, span.into())
    }
}
