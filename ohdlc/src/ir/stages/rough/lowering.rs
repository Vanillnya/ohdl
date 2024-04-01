use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        modules::Module,
        name_resolution::Import,
        resolving::Resolvable,
        types::{Entity, Enum, Record, Type, TypeId, Variant},
        ScopeId,
    },
    span::Spanned,
};

use super::RoughIR;

pub struct RoughLowering<'ir> {
    pub arena: &'ir Bump,
    pub ir: RoughIR<'ir>,
}

impl<'ir> RoughLowering<'ir> {
    pub fn lower(arena: &'ir Bump, root: &[Spanned<ast::Item<'_>>]) -> Self {
        let mut lowering = RoughLowering {
            arena,
            ir: RoughIR::new(),
        };
        for item in root {
            lowering.lower_item(lowering.ir.resolving_scopes.root, item);
        }
        lowering
    }

    pub fn lower_item(&mut self, scope: ScopeId, item: &ast::Item<'_>) {
        match item {
            ast::Item::Use(u) => self.lower_use(scope, u),
            ast::Item::Module(m) => self.lower_mod(scope, m),
            ast::Item::Entity(e) => self.introduce_type(scope, |type_id| {
                Type::Entity(Entity {
                    type_id,
                    name: e.name,
                })
            }),
            ast::Item::Record(r) => self.introduce_type(scope, |type_id| {
                Type::Record(Record {
                    type_id,
                    name: r.name,
                })
            }),
            ast::Item::Enum(e) => self.introduce_type(scope, |type_id| {
                Type::Enum(Enum {
                    type_id,
                    name: e.name,
                    variants: self
                        .arena
                        .alloc_slice_fill_iter(e.variants.iter().map(|&ident| Variant { ident })),
                })
            }),
            ast::Item::Arch(_) => {}
        }
    }

    fn lower_use(&mut self, scope: ScopeId, u: &ast::Use) {
        let path = &u.path.0;
        let import = Import {
            src: scope,
            path: self
                .arena
                .alloc_slice_fill_iter(path.iter().map(|seg| seg.0)),
        };
        // TODO: safe that somewhere
        println!("{import:?}");
    }

    fn lower_mod(&mut self, scope: ScopeId, m: &ast::Module<'_>) {
        let sub_scope = self.ir.resolving_scopes.sub_scope(scope);

        let module = self.ir.modules.insert(Module {
            name: m.name,
            scope: sub_scope,
        });
        self.ir.introduce(scope, m.name, Resolvable::Module(module));

        for i in &m.items {
            self.lower_item(sub_scope, i);
        }
    }

    fn introduce_type<F>(&mut self, scope: ScopeId, f: F)
    where
        F: FnOnce(TypeId) -> Type<'ir>,
    {
        let id = self.ir.types.insert_with(f);

        let name = self.ir.types[id].name();
        self.ir.introduce(scope, name, Resolvable::Type(id));
    }
}
