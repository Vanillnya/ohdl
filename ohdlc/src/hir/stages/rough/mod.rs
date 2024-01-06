use bumpalo::Bump;

use crate::{
    ast::{self},
    hir::{
        modules::Module,
        resolving::{Resolvable, ScopeId},
        types::{Entity, Enum, Record, Type, Variant},
        HIR,
    },
    span::Spanned,
};

pub struct RoughLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> RoughLowering<'_, 'hir> {
    pub fn lower(&mut self, root: &[Spanned<ast::Item<'_>>]) {
        for item in root {
            self.lower_item(self.hir.resolving_scopes.root(), item);
        }
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
        self.hir.introduce(
            scope,
            u.path.0.last().unwrap().0,
            Resolvable::Using(
                self.arena
                    .alloc_slice_fill_iter(u.path.0.iter().map(|seg| seg.0)),
            ),
        );
    }

    fn lower_mod(&mut self, scope: ScopeId, m: &ast::Module<'_>) {
        let sub_scope = self.hir.resolving_scopes.sub_scope(scope);

        let module = self.hir.modules.insert(Module {
            name: m.name,
            scope: sub_scope,
        });
        self.hir
            .introduce(scope, m.name, Resolvable::Module(module));

        for i in &m.items {
            self.lower_item(sub_scope, i);
        }
    }

    fn introduce_type<F>(&mut self, scope: ScopeId, f: F)
    where
        F: FnOnce(usize) -> Type<'hir>,
    {
        let entry = self.hir.types.vacant_entry();
        let id = entry.key();
        entry.insert(f(id));

        let name = self.hir.types[id].name();
        self.hir.introduce(scope, name, Resolvable::Type(id));
    }
}
