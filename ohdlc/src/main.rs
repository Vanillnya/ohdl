#![deny(rust_2018_idioms)]

use ariadne::{Label, Report};
use bumpalo::Bump;
use message::Messages;
use parser::Parser;

use crate::{
    ir::{
        stages::{resolve::ResolveLowering, rough::lowering::RoughLowering},
        IR,
    },
    lexer::Lexer,
};

mod ast;
mod ir;
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

    let ir_arena = Bump::new();

    let root = parser.parse();
    report_messages(&source);
    let root = root?;

    let mut ir = IR::new();

    {
        let rough = RoughLowering {
            arena: &ir_arena,
            ir: &mut ir,
        };
        rough.lower(&root);
        report_messages(&source);
    }

    {
        let resolve = ResolveLowering {
            arena: &ir_arena,
            ir: &mut ir,
        };
        resolve.lower();
        report_messages(&source);
    }

    for import in ir.name_resolution.imports.values() {
        println!("{import:?}");
    }

    Ok(())
}

fn report_messages(source: &Source<'_>) {
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
