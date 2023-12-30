use std::collections::HashMap;

use bumpalo::Bump;

use crate::{
    ast::{self, Item},
    hir::{Declaration, Enum, Record, TypeResolvingScope, Variant, HIR},
};

pub struct RoughLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> RoughLowering<'_, 'hir> {
    pub fn lower(&mut self, items: &[ast::Item]) {
        let root_scope = self.hir.tr_scopes.insert(TypeResolvingScope {
            parent: None,
            types: HashMap::new(),
        });
        for item in items {
            self.lower_item(item, root_scope);
        }
    }

    pub fn lower_item(&mut self, item: &Item, scope: usize) {
        match &item.base.0 {
            ast::ItemBase::Record(r) => {
                let id = self.hir.types.insert_with(|type_id| {
                    Declaration::Record(Record {
                        type_id,
                        name: r.name,
                    })
                });
                self.hir.tr_scopes[scope].types.insert(r.name.0, id); // TODO: check for collision
            }
            ast::ItemBase::Enum(e) => {
                let id = self.hir.types.insert_with(|type_id| {
                    Declaration::Enum(Enum {
                        type_id,
                        name: e.name,
                        variants: self.arena.alloc_slice_fill_iter(
                            e.variants.iter().map(|&ident| Variant { ident }),
                        ),
                    })
                });
                self.hir.tr_scopes[scope].types.insert(e.name.0, id); // TODO: check for collision
            }
            _ => { /* TODO */ }
        }
    }
}
