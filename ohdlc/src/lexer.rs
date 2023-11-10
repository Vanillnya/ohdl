use ariadne::ReportKind;
use logos::Logos;

use crate::{
    ast::span::{Spanned, WithSpan},
    message::{Message, Messages},
};

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum TokenKind {
    #[token("entity")]
    KwEntity,
    #[token("in")]
    KwIn,
    #[token("out")]
    KwOut,
    #[token("arch")]
    KwArch,
    #[token("for")]
    KwFor,

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
    Ident,

    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
}

pub struct Lexer(pub Vec<Spanned<TokenKind>>);

impl Lexer {
    pub fn new(text: &str) -> Result<Self, Messages> {
        let tokenizer = TokenKind::lexer(text);

        let mut tokens = Vec::new();
        let mut messages = Messages(Vec::new());

        for token in tokenizer.spanned() {
            match token {
                (Ok(token), span) => tokens.push(token.with_span(span)),
                (Err(_), span) => messages.0.push(Message {
                    kind: ReportKind::Error,
                    span: span.into(),
                    message: "Unknown Token".to_string(),
                    label_message: "Whatever this is here".to_string(),
                }),
            }
        }

        if messages.0.is_empty() {
            Ok(Self(tokens))
        } else {
            Err(messages)
        }
    }
}