use std::collections::{hash_map::Entry, HashMap};

use bumpalo::Bump;

use crate::{
    ast::{self},
    hir::{
        modules::Module,
        resolving::{Resolvable, ResolvingScope},
        types::{Entity, Enum, Record, Type, Variant},
        HIR,
    },
    message::Message,
    span::Spanned,
    symbol::Ident,
    MESSAGES,
};

pub struct RoughLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> RoughLowering<'_, 'hir> {
    pub fn lower(&mut self, root: &[Spanned<ast::Item>]) {
        let root_scope = self.hir.resolving_scopes.insert(ResolvingScope {
            parent: None,
            entries: HashMap::new(),
        });
        for item in root {
            self.lower_item(root_scope, item);
        }
    }

    pub fn lower_item(&mut self, scope: usize, item: &ast::Item) {
        match item {
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
            _ => { /* TODO */ }
        }
    }

    pub fn lower_mod(&mut self, scope: usize, m: &ast::Module) {
        let sub_scope = self.hir.resolving_scopes.sub_scope(scope);

        let module = self.hir.modules.insert(Module {
            name: m.name,
            scope: sub_scope,
        });
        self.introduce(scope, m.name, Resolvable::Module(module));

        for i in &m.items {
            self.lower_item(sub_scope, i);
        }
    }

    fn introduce_type<F>(&mut self, scope: usize, f: F)
    where
        F: FnOnce(usize) -> Type<'hir>,
    {
        let entry = self.hir.types.vacant_entry();
        let id = entry.key();
        entry.insert(f(id));

        let name = self.hir.types[id].name();
        self.introduce(scope, name, Resolvable::Type(id));
    }

    fn introduce(&mut self, scope: usize, name: Ident, resolvable: Resolvable) {
        match self.hir.resolving_scopes[scope].entries.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert(resolvable);
            }
            Entry::Occupied(entry) => {
                let original = match *entry.get() {
                    Resolvable::Type(t) => self.hir.types[t].name(),
                    Resolvable::Module(m) => self.hir.modules[m].name,
                };
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }
}
