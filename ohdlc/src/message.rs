use ariadne::ReportKind;

use crate::{ast::span::Span, lexer::TokenKind, parser::PResult};

pub struct Messages<'s>(pub Vec<Message<'s>>);

impl<'s> Messages<'s> {
    #[inline(always)]
    pub fn report<T>(&mut self, message: Message<'s>) -> PResult<T> {
        self.0.push(message);
        Err(())
    }
}

pub struct Message<'s> {
    pub kind: ReportKind<'s>,
    pub span: Span,
    pub message: String,
    pub label_message: String,
}

impl Message<'_> {
    pub fn unexpected_end(span: impl Into<Span>) -> Self {
        Self {
            kind: ReportKind::Error,
            span: span.into(),
            message: "Unexpected end of stream".to_owned(),
            label_message: "Stream ended here".to_owned(),
        }
    }

    pub fn unexpected_token(span: impl Into<Span>, expected: TokenKind, got: TokenKind) -> Self {
        Message {
            kind: ReportKind::Error,
            span: span.into(),
            message: format!("Expected {expected:?}, but got {got:?}"),
            label_message: "Wrong token kind here".to_string(),
        }
    }
}