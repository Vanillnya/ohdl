#![deny(rust_2018_idioms)]

use ariadne::{Label, Report};
use bumpalo::Bump;

use ohdlc::{
    ir::{
        import_bucket::ImportBucket,
        name_lookup::NameLookup,
        registries::{ModuleRegistry, RoughArchRegistry, RoughEntityRegistry, RoughTypeRegistry},
        stages::{
            architectures::ArchitectureStage, flatten_lookup::FlattenLookupStage,
            refine::RefineStage, rough::RoughStage,
        },
    },
    lexer::Lexer,
    parser::Parser,
    Source, MESSAGES,
};

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
    let mut module_reg = ModuleRegistry::default();
    let mut type_reg = RoughTypeRegistry::default();
    let mut entity_reg = RoughEntityRegistry::default();
    let mut arch_reg = RoughArchRegistry::default();
    let mut name_lookup = NameLookup::new();
    let mut import_bucket = ImportBucket::new();

    {
        let rough = RoughStage {
            arena: &ir_arena,
            name_lookup: &mut name_lookup,
            import_bucket: &mut import_bucket,
            module_reg: &mut module_reg,
            type_reg: &mut type_reg,
            entity_reg: &mut entity_reg,
            arch_reg: &mut arch_reg,
            root: &root,
        };
        rough.lower();
        report_messages(&source);
    }

    let name_lookup = {
        let resolve = FlattenLookupStage {
            module_reg: &module_reg,
            name_lookup,
            import_bucket,
            resolvables: Vec::new(),
        };
        let lookup = resolve.lower();
        report_messages(&source);
        lookup
    };
    let name_lookup = name_lookup.unwrap();

    let (type_reg, entity_reg) = {
        let refine = RefineStage {
            arena: &ir_arena,
            name_lookup: &name_lookup,
            module_registry: &module_reg,
        };

        let type_reg = refine.refine_types(type_reg);
        report_messages(&source);

        let entity_reg = refine.refine_entities(entity_reg);
        report_messages(&source);

        (type_reg, entity_reg)
    };

    println!("{type_reg:#?}");
    println!("{entity_reg:#?}");

    {
        let architectures = ArchitectureStage {
            arena: &ir_arena,
            name_lookup: &name_lookup,
            module_reg: &module_reg,
            arch_reg: &arch_reg,
        };
        architectures.lower();
        report_messages(&source);
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
