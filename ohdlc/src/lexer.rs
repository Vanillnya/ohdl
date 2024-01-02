use logos::Logos;

use crate::{
    message::Message,
    span::{Spanned, WithSpan},
    MESSAGES,
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
    #[token("use")]
    KwUse,
    #[token("mod")]
    KwMod,
    #[token("record")]
    KwRecord,
    #[token("enum")]
    KwEnum,
    #[token("place")]
    KwPlace,
    #[token("signal")]
    KwSignal,

    #[token("and")]
    KwAnd,
    #[token("or")]
    KwOr,
    #[token("nand")]
    KwNand,
    #[token("nor")]
    KwNor,
    #[token("xor")]
    KwXor,
    #[token("xnor")]
    KwXnor,
    #[token("not")]
    KwNot,

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
    Ident,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,
    #[token("::")]
    ColonColon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("<=")]
    LeftBigArrow,
    #[token("=>")]
    RightBigArrow,
}

pub struct Lexer(pub Vec<Spanned<TokenKind>>);

impl Lexer {
    pub fn new(text: &str) -> Result<Self, ()> {
        let tokenizer = TokenKind::lexer(text);

        let mut poisoned = false;
        let mut tokens = Vec::new();

        for token in tokenizer.spanned() {
            match token {
                (Ok(token), span) => tokens.push(token.with_span(span)),
                (Err(_), span) => {
                    MESSAGES.report(Message::unknown_token(span));
                    poisoned = true;
                }
            }
        }

        if poisoned {
            Err(())
        } else {
            Ok(Self(tokens))
        }
    }
}
