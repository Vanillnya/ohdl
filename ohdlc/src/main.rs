use std::ops::Range;

use ariadne::{Label, Report, ReportKind};
use ast::Span;
use bumpalo::Bump;
use message::Messages;
use parser::Parser;

use crate::lexer::Lexer;

mod ast;
pub mod hir;
mod lexer;
mod message;
mod parser;

pub struct Source<'s>(pub String, pub &'s str);

fn main() -> Result<(), ()> {
    let source = Source("work.ohd".to_owned(), include_str!("work.ohd"));

    println!("[STAGE] Lexer");

    let lexer = Lexer::new(&source.1);
    let lexer = finish_stage(&source, lexer)?;

    println!("[STAGE] Parser");
    let mut parser = Parser::new(source, lexer);

    println!("{:#?}", parser.parse_item());
    let record = parser.parse_item();
    println!("{:#?}", record);
    println!("{:#?}", parser.parse_item());
    println!("{:#?}", parser.parse_item());
    println!("{:#?}", parser.parse_item());

    for msg in parser.messages.0 {
        print_report(
            &parser.source,
            msg.kind,
            msg.span,
            msg.message,
            msg.label_message,
        );
    }

    let mut hir_arena = Bump::new();
    hir::Item::represent(&mut hir_arena, record?);

    Ok(())
}

fn finish_stage<T>(source: &Source, input: Result<T, Messages>) -> Result<T, ()> {
    input.map_err(|msgs| {
        for msg in msgs.0 {
            print_report(source, msg.kind, msg.span, msg.message, msg.label_message);
        }
        ()
    })
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
