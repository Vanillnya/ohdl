use std::ops::Range;

use ariadne::ReportKind;
use logos::SpannedIter;

use crate::{ast_span::Span, print_report, Source, TokenValue};

pub mod decl;

type TokenIter<'source> = itertools::PeekNth<SpannedIter<'source, TokenValue>>;

pub struct Token(pub TokenValue, pub Span);
pub struct TokenRef<'r>(pub &'r TokenValue, pub Span);

pub struct Parser<'s> {
    pub source: Source<'s>,
    tokens: TokenIter<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: Source<'s>, tokens: SpannedIter<'s, TokenValue>) -> Self {
        Self {
            source,
            tokens: itertools::peek_nth(tokens),
        }
    }

    #[inline(always)]
    pub fn current(&mut self) -> Result<Option<TokenRef>, ()> {
        match self.tokens.peek() {
            Some((Ok(value), rng)) => Ok(Some(TokenRef(value, Span::from(rng)))),
            Some((Err(_), span)) => {
                let span = Span::from(span);
                print_report(
                    &self.source,
                    ReportKind::Error,
                    span,
                    "Unknown Token",
                    "Whatever this is here",
                );
                Err(())
            }
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> Result<Option<Token>, ()> {
        match self.tokens.next() {
            Some((Ok(value), rng)) => Ok(Some(Token(value, Span::from(rng)))),
            Some((Err(_), span)) => {
                let span = Span::from(span);
                print_report(
                    &self.source,
                    ReportKind::Error,
                    span,
                    "Unknown Token",
                    "Whatever this is here",
                );
                Err(())
            }
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn slice(&self, span: Span) -> &'s str {
        unsafe { self.source.1.get_unchecked::<Range<usize>>(span.into()) }
    }
}

pub trait Parselet<'s>: Sized {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ()>;
}
