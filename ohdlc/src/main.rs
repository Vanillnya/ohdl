use std::ops::Range;

use ariadne::{Label, Report, ReportKind};
use message::{Message, Messages};
use parser::Parser;
use span::Span;

use crate::{hir::lowering::HIR, lexer::Lexer};

mod ast;
pub mod hir;
mod lexer;
mod message;
mod parser;
pub mod span;

#[derive(Clone)]
pub struct Source<'s>(pub String, pub &'s str);

fn main() -> Result<(), ()> {
    let messages = Box::leak(Box::new(Messages::new()));

    let source = Source("work.ohd".to_owned(), include_str!("work.ohd"));

    println!("[STAGE] Lexer");

    let lexer = Lexer::new(messages, &source.1);
    report_messages(&source, messages);
    let lexer = lexer?;

    println!("[STAGE] Parser");

    let mut parser = Parser::new(messages, source.clone(), lexer);

    let hir = HIR::new();

    for _ in 0..5 {
        let item = parser.parse_item()?;
        let hir_item = hir.lower_item(item);
        println!("{hir_item:#?}");
    }

    report_messages(&source, messages);

    Ok(())
}

fn report_messages(source: &Source, messages: &'static Messages) {
    messages.drain(|msg| report_to_stdout(source, msg));
}

fn report_to_stdout(source: &Source, msg: Message) {
    print_report(source, msg.kind, msg.span, msg.message, msg.label_message);
}

fn print_report(
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
