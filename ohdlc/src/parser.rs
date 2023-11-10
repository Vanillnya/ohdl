use std::ops::Range;

use crate::{
    ast::{
        span::{Span, Spanned},
        Ident,
    },
    lexer::{Lexer, TokenKind},
    message::{Message, Messages},
    Source,
};

pub mod item;

pub type PResult<T> = Result<T, ()>;

pub struct Parser<'s> {
    pub source: Source<'s>,
    lexer: Lexer,
    cursor: usize,
    pub messages: Messages<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: Source<'s>, lexer: Lexer) -> Self {
        Self {
            source,
            lexer,
            cursor: 0,
            messages: Messages(Vec::new()),
        }
    }

    #[inline(always)]
    fn current(&mut self) -> PResult<Spanned<TokenKind>> {
        match self.lexer.0.get(self.cursor) {
            Some(val) => Ok(*val),
            None => self.messages.report(Message::unexpected_end(
                self.source.1.len()..self.source.1.len(),
            ))?,
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
    fn span_begin(&mut self) -> usize {
        match self.lexer.0.get(self.cursor) {
            Some(Spanned(_, span)) => span.0,
            None => self.source.1.len(),
        }
    }

    #[inline(always)]
    fn span_end(&mut self, begin: usize) -> Span {
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
            self.messages
                .report(Message::unexpected_token(token.1, kind, token.0))?
        }
    }

    fn ident(&mut self) -> PResult<Ident<'s>> {
        self.consume(TokenKind::Ident)
            .map(|Spanned(_, span)| Spanned(self.slice(span), span))
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

    fn prev_span(&self) -> Span {
        self.lexer.0[self.cursor - 1].1
    }
}
