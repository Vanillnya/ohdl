use std::sync::Mutex;

use ariadne::ReportKind;

use crate::{lexer::TokenKind, span::Span, symbol::Ident};

pub struct Messages {
    messages: Mutex<Vec<Message>>,
}

impl Messages {
    pub const fn new() -> Self {
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
    pub message: String,
    pub labels: Vec<Label>,
    pub location: Span,
}

pub struct Label {
    pub span: Span,
    pub message: String,
}

impl Message {
    pub fn unknown_token(span: impl Into<Span>) -> Self {
        let span = span.into();
        Self {
            kind: ReportKind::Error,
            message: "Unknown Token".to_owned(),
            labels: vec![Label {
                span,
                message: "Whatever this is here".to_owned(),
            }],
            location: span,
        }
    }

    pub fn unexpected_end(span: impl Into<Span>) -> Self {
        let span = span.into();
        Self {
            kind: ReportKind::Error,
            message: "Unexpected end of stream".to_owned(),
            labels: vec![Label {
                span,
                message: "Stream ended here".to_owned(),
            }],
            location: span,
        }
    }

    pub fn unexpected_token(
        span: impl Into<Span>,
        expected: impl ToString,
        got: TokenKind,
    ) -> Self {
        let span = span.into();
        let expected = expected.to_string();
        Message {
            kind: ReportKind::Error,
            message: format!("Expected {expected}, but got {got:?}"),
            labels: vec![Label {
                span,
                message: "Wrong token kind here".to_owned(),
            }],
            location: span,
        }
    }

    pub fn already_in_scope(
        name: impl ToString,
        second: impl Into<Span>,
        first: impl Into<Span>,
    ) -> Self {
        let first = first.into();
        let second = second.into();
        let name = name.to_string();
        Message {
            kind: ReportKind::Error,
            message: format!("{name} is already in scope"),
            labels: vec![
                Label {
                    span: second,
                    message: "This is the second definition".to_owned(),
                },
                Label {
                    span: first,
                    message: "First introduced here".to_owned(),
                },
            ],
            location: second,
        }
    }

    pub fn use_continues_after_type(ty: impl Into<Span>) -> Self {
        let ty = ty.into();
        Message {
            kind: ReportKind::Error,
            message: "`use` continues after type".to_owned(),
            labels: vec![Label {
                span: ty,
                message: "Type should've ended use here".to_owned(),
            }],
            location: ty,
        }
    }

    pub fn could_not_resolve(resolvable: Ident) -> Self {
        let val = resolvable.0.get();
        let span = resolvable.1;
        Message {
            kind: ReportKind::Error,
            message: format!("Could not resolve {val}"),
            labels: vec![Label {
                span,
                message: "Can't resolve this".to_owned(),
            }],
            location: span,
        }
    }

    pub fn stuck_on_import(import_segment: Ident, sub_import: Span) -> Self {
        let segment_span = import_segment.1;
        let segment = import_segment.0.get();
        Message {
            kind: ReportKind::Error,
            message: format!("Stuck on importing {segment}"),
            labels: vec![
                Label {
                    span: segment_span,
                    message: "This import can't get any further...".to_owned(),
                },
                Label {
                    span: sub_import,
                    message: "..because this import doesn't make any progress".to_owned(),
                },
            ],
            location: segment_span,
        }
    }
}
