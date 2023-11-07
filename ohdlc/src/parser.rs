use std::ops::Range;

use ariadne::ReportKind;
use logos::SpannedIter;

use crate::{
    ast::span::{Span, Spanned, WithSpan},
    Source, TokenValue,
};

pub mod item;

type TokenIter<'source> = itertools::PeekNth<SpannedIter<'source, TokenValue>>;

pub struct Message<'s> {
    pub kind: ReportKind<'s>,
    pub span: Span,
    pub message: String,
    pub label_message: String,
}

pub struct Messages<'s>(pub Vec<Message<'s>>);

impl<'s> Messages<'s> {
    #[inline(always)]
    pub fn report<T>(&mut self, message: Message<'s>) -> PResult<T> {
        self.0.push(message);
        Err(())
    }
}

pub type PResult<T> = Result<T, ()>;

pub struct Parser<'s> {
    pub source: Source<'s>,
    tokens: TokenIter<'s>,
    pub messages: Messages<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: Source<'s>, tokens: SpannedIter<'s, TokenValue>) -> Self {
        Self {
            source,
            tokens: itertools::peek_nth(tokens),
            messages: Messages(Vec::new()),
        }
    }

    #[inline(always)]
    pub fn current(&mut self) -> PResult<Option<Spanned<&TokenValue>>> {
        match self.tokens.peek() {
            Some((Ok(value), rng)) => Ok(Some(value.with_span(rng.into()))),
            Some((Err(_), span)) => self.messages.report(Message {
                kind: ReportKind::Error,
                span: span.into(),
                message: "Unknown Token".to_string(),
                label_message: "Whatever this is here".to_string(),
            })?,
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> PResult<Option<Spanned<TokenValue>>> {
        match self.tokens.next() {
            Some((Ok(value), rng)) => Ok(Some(value.with_span(rng.into()))),
            Some((Err(_), span)) => self.messages.report(Message {
                kind: ReportKind::Error,
                span: span.into(),
                message: "Unknown Token".to_string(),
                label_message: "Whatever this is here".to_string(),
            })?,
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn slice(&self, span: Span) -> &'s str {
        unsafe { self.source.1.get_unchecked::<Range<usize>>(span.into()) }
    }

    #[inline(always)]
    pub fn span_begin(&mut self) -> PResult<usize> {
        Span::start(self)
    }

    #[inline(always)]
    pub fn span_end(&mut self, begin: usize) -> PResult<Span> {
        Span::with_start(self, begin)
    }
}
