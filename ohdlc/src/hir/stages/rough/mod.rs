use std::collections::{hash_map::Entry, HashMap};

use bumpalo::Bump;

use crate::{
    ast::{self, Item},
    hir::{
        type_resolving::TypeResolvingScope,
        types::{Entity, Enum, Record, Type, Variant},
        HIR,
    },
    message::Message,
    MESSAGES,
};

pub struct RoughLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> RoughLowering<'_, 'hir> {
    pub fn lower_module(&mut self, module: &ast::Module) {
        let root_scope = self.hir.tr_scopes.insert(TypeResolvingScope {
            parent: None,
            types: HashMap::new(),
        });
        for item in &module.items {
            self.lower_item(item, root_scope);
        }
    }

    pub fn lower_item(&mut self, item: &Item, scope: usize) {
        match item {
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

    fn introduce_type<F>(&mut self, scope: usize, f: F)
    where
        F: FnOnce(usize) -> Type<'hir>,
    {
        let entry = self.hir.types.vacant_entry();
        let id = entry.key();
        entry.insert(f(id));

        let name = self.hir.types[id].name();
        match self.hir.tr_scopes[scope].types.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert(id);
            }
            Entry::Occupied(entry) => {
                let original = self.hir.types[*entry.get()].name();
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }
}
