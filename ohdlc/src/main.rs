use ariadne::{Label, Report};
use bumpalo::Bump;
use message::Messages;
use parser::Parser;

use crate::{
    hir::{stages::rough::RoughLowering, HIR},
    lexer::Lexer,
};

mod ast;
mod hir;
mod lexer;
mod message;
mod parser;
mod span;
mod symbol;

static MESSAGES: Messages = Messages::new();

#[derive(Clone)]
pub struct Source<'s>(pub String, pub &'s str);

fn main() -> Result<(), ()> {
    let source = Source("work.ohd".to_owned(), include_str!("work.ohd"));

    println!("[STAGE] Lexer");

    let lexer = Lexer::new(&source.1);
    report_messages(&source);
    let lexer = lexer?;

    println!("[STAGE] Parser");

    let parser_arena = Bump::new();

    let mut parser = Parser::new(&parser_arena, source.clone(), lexer);

    let hir_arena = Bump::new();
    let mut hir = HIR::new();
    let mut rough = RoughLowering {
        arena: &hir_arena,
        hir: &mut hir,
    };

    rough.lower(&[
        parser.parse_item().unwrap(),
        parser.parse_item().unwrap(),
        parser.parse_item().unwrap(),
        parser.parse_item().unwrap(),
    ]);

    println!("{hir:#?}");

    report_messages(&source);

    Ok(())
}

fn report_messages(source: &Source) {
    MESSAGES.drain(|msg| {
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
