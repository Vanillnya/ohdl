#![deny(rust_2018_idioms)]

use ariadne::{Label, Report};
use bumpalo::Bump;
use message::Messages;
use parser::Parser;

use crate::{
    ir::{
        import_bucket::ImportBucket,
        name_lookup::NameLookup,
        registry::Registry,
        stages::{flatten_lookup::FlattenLookupStage, rough::RoughStage},
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

    let root = parser.parse();
    let root = match root {
        Ok(tree) => tree,
        Err(messages) => {
            MESSAGES.extend(messages);
            report_messages(&source);
            return Err(());
        }
    };

    let ir_arena = Bump::new();
    let mut registry = Registry::default();
    let mut name_lookup = NameLookup::new();
    let mut import_bucket = ImportBucket::new();

    {
        let rough = RoughStage {
            arena: &ir_arena,
            registry: &mut registry,
            name_lookup: &mut name_lookup,
            import_bucket: &mut import_bucket,
        };
        rough.lower(&root);
        report_messages(&source);
    }

    let name_lookup = {
        let resolve = FlattenLookupStage {
            registry: &registry,
            name_lookup,
            import_bucket,
            resolvables: Vec::new(),
        };
        let lookup = resolve.lower();
        report_messages(&source);
        lookup
    };

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
