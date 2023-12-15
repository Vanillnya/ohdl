use bumpalo::Bump;

use crate::ast;

use super::*;

pub struct RIR {
    arena: Bump,
}

impl RIR {
    pub fn new() -> Self {
        Self { arena: Bump::new() }
    }

    pub fn lower_item(&self, item: ast::Item) -> Option<Item> {
        let Some(base) = self.lower_item_base(item.base.0) else {
            return None;
        };
        Some(Item {
            base_span: item.base.1,
            base,
        })
    }

    fn lower_item_base(&self, base: ast::ItemBase) -> Option<ItemBase> {
        match base {
            ast::ItemBase::Use(_) => None,
            ast::ItemBase::Entity(e) => Some(ItemBase::Entity(Ident::intern(e.name.0))),
            ast::ItemBase::Arch(a) => Some(ItemBase::Arch(Ident::intern(a.name.0))),
            ast::ItemBase::Record(r) => Some(ItemBase::Record(Ident::intern(r.name.0))),
            ast::ItemBase::Enum(e) => Some(ItemBase::Record(Ident::intern(e.name.0))),
        }
    }
}
