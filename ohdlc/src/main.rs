use logos::Logos;
use parser::{decl::Decl, Parselet, Parser};

mod parser;

pub struct Source<'s>(pub String, pub &'s str);

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum TokenValue {
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

fn main() {
    let text = include_str!("work.ohd");
    let lexer = TokenValue::lexer(text);
    let mut parser = Parser::new(Source("work.ohd".to_owned(), text), lexer.spanned());
    let decl = Decl::parse(&mut parser);
}
