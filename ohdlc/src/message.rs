use std::sync::Mutex;

use ariadne::ReportKind;

use crate::{lexer::TokenKind, span::Span};

pub struct Messages {
    messages: Mutex<Vec<Message>>,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new(Vec::new()),
        }
    }

    #[inline]
    pub fn report(&self, message: Message) {
        self.messages.lock().unwrap().push(message);
    }

    pub fn drain<F>(&self, f: F)
    where
        F: Fn(Message),
    {
        self.messages.lock().unwrap().drain(..).for_each(f)
    }
}

pub struct Message {
    pub kind: ReportKind<'static>,
    pub span: Span,
    pub message: String,
    pub label_message: String,
}

impl Message {
    pub fn unexpected_end(span: impl Into<Span>) -> Self {
        Self {
            kind: ReportKind::Error,
            span: span.into(),
            message: "Unexpected end of stream".to_owned(),
            label_message: "Stream ended here".to_owned(),
        }
    }

    pub fn unexpected_token(
        span: impl Into<Span>,
        expected: impl ToString,
        got: TokenKind,
    ) -> Self {
        let expected = expected.to_string();
        Message {
            kind: ReportKind::Error,
            span: span.into(),
            message: format!("Expected {expected}, but got {got:?}"),
            label_message: "Wrong token kind here".to_string(),
        }
    }
}
