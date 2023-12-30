use bumpalo::Bump;

use crate::{
    ast::{self, Item},
    hir::{Declaration, Enum, Variant, HIR},
};

pub struct RoughLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> RoughLowering<'_, 'hir> {
    pub fn lower(&mut self, items: &[ast::Item]) {
        for item in items {
            self.lower_item(item);
        }
    }

    pub fn lower_item(&mut self, item: &Item) {
        match &item.base.0 {
            ast::ItemBase::Enum(e) => {
                self.hir.types.insert_with(|type_id| {
                    Declaration::Enum(Enum {
                        type_id,
                        name: e.name,
                        variants: self.arena.alloc_slice_fill_iter(
                            e.variants.iter().map(|&ident| Variant { ident }),
                        ),
                    })
                });
            }
            _ => { /* TODO */ }
        }
    }
}
