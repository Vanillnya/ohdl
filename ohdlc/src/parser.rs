use std::ops::Range;

use bumpalo::Bump;

use crate::{
    lexer::{Lexer, TokenKind},
    message::Message,
    span::{Span, Spanned, WithSpan},
    symbol::{Ident, Symbol},
    Source, MESSAGES,
};

pub mod expr;
pub mod item;
pub mod module;
pub mod stmt;
pub mod ty;

pub type PResult<T> = Result<T, ()>;

pub struct Parser<'s, 'a> {
    pub arena: &'a Bump,
    pub source: Source<'s>,
    lexer: Lexer,
    cursor: usize,
}

impl<'s, 'a> Parser<'s, 'a> {
    pub fn new(arena: &'a Bump, source: Source<'s>, lexer: Lexer) -> Self {
        Self {
            arena,
            source,
            lexer,
            cursor: 0,
        }
    }

    #[inline(always)]
    fn has_next(&self) -> bool {
        self.lexer.0.len() > self.cursor
    }

    #[inline(always)]
    fn current(&mut self) -> PResult<Spanned<TokenKind>> {
        match self.lexer.0.get(self.cursor) {
            Some(val) => Ok(*val),
            None => {
                MESSAGES.report(Message::unexpected_end(
                    self.source.1.len()..self.source.1.len(),
                ));
                Err(())
            }
        }
    }

    #[inline(always)]
    fn next(&mut self) -> PResult<Spanned<TokenKind>> {
        let val = self.current()?;
        self.bump();
        Ok(val)
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.cursor += 1;
    }

    #[inline(always)]
    fn slice(&self, span: Span) -> &'s str {
        unsafe { self.source.1.get_unchecked::<Range<usize>>(span.into()) }
    }

    #[inline(always)]
    fn span_enter(&mut self) -> usize {
        match self.lexer.0.get(self.cursor) {
            Some(Spanned(_, span)) => span.0,
            None => self.source.1.len(),
        }
    }

    #[inline(always)]
    fn span_leave(&mut self, begin: usize) -> Span {
        let end = match self.lexer.0.get(self.cursor) {
            Some(Spanned(_, span)) => span.1,
            None => self.source.1.len(),
        };
        Span(begin, end)
    }

    fn consume(&mut self, kind: TokenKind) -> PResult<Spanned<TokenKind>> {
        let token = self.next()?;
        if token.0 == kind {
            Ok(token)
        } else {
            MESSAGES.report(Message::unexpected_token(
                token.1,
                format!("{kind:?}"),
                token.0,
            ));
            Err(())
        }
    }

    fn ident(&mut self) -> PResult<Ident> {
        self.consume(TokenKind::Ident)
            .map(|Spanned(_, span)| Symbol::intern(self.slice(span)).with_span(span))
    }

    fn eat_token(&mut self, kind: TokenKind) -> PResult<bool> {
        let current = self.current()?;
        if current.0 == kind {
            self.bump();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn kind(&mut self) -> PResult<TokenKind> {
        Ok(self.current()?.0)
    }
}

#[macro_export]
macro_rules! spanned {
    ($self:ident { $($calc:tt)* }) => {
        {
            use crate::span::WithSpan;
            let span = $self.span_enter();
            let val = { $($calc)* };
            let span = $self.span_leave(span);
            val.with_span(span)
        }
    };
}
