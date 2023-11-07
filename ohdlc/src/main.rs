use std::ops::Range;

use ariadne::{Label, Report, ReportKind};
use ast::span::Span;
use logos::Logos;
use parser::Parser;

mod ast;
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
    let item = parser.parse_item();
    println!("{item:#?}");
}

pub fn print_report(
    source: &Source,
    report_kind: ReportKind,
    span: Span,
    message: impl Into<String>,
    label_message: impl Into<String>,
) {
    let span: Range<usize> = span.into();

    let filename = source.0.as_str();

    let report = Report::build(report_kind, filename, span.start)
        .with_message(message.into())
        .with_label(Label::new((filename, span.into())).with_message(label_message.into()))
        .finish();

    report
        .eprint((filename, ariadne::Source::from(source.1)))
        .unwrap();
}
