use std::collections::HashMap;

use ariadne::{Label, Report};
use bumpalo::Bump;
use message::Messages;
use parser::Parser;

use crate::{
    lexer::Lexer,
    rir::{lowering::RIR, Scope},
};

mod ast;
mod lexer;
mod message;
mod parser;
pub mod rir;
pub mod span;
pub mod symbol;

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

    let parser_arena = Bump::new();

    let mut parser = Parser::new(&parser_arena, messages, source.clone(), lexer);

    let mut hir = RIR::new(messages);

    let mut scope = Scope {
        parent: None,
        entries: HashMap::new(),
    };
    for _ in 0..7 {
        let item = parser.parse_item()?;
        let _ = hir.lower_item(&mut scope, item);
    }
    println!("{scope:#?}");

    report_messages(&source, messages);

    Ok(())
}

fn report_messages(source: &Source, messages: &'static Messages) {
    messages.drain(|msg| {
        let filename = source.0.as_str();

        let report =
            Report::build(msg.kind, filename, msg.location.0)
                .with_message(msg.message)
                .with_labels(msg.labels.into_iter().map(|label| {
                    Label::new((filename, label.span.into())).with_message(label.message)
                }))
                .finish();

        report
            .eprint((filename, ariadne::Source::from(source.1)))
            .unwrap();
    });
}
