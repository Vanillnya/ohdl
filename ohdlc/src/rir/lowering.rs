use std::collections::hash_map;

use bumpalo::Bump;

use crate::{
    ast,
    message::{Message, Messages},
};

use super::*;

pub type LResult<T> = Result<T, ()>;
pub type EntryIdx = usize;

pub struct RIR {
    arena: Bump,
    entry_pool: Vec<Entry>,
    messages: &'static Messages,
}

impl RIR {
    pub fn new(messages: &'static Messages) -> Self {
        Self {
            arena: Bump::new(),
            entry_pool: Vec::new(),
            messages,
        }
    }

    pub fn lower_item(&mut self, scope: &mut Scope, item: ast::Item) -> LResult<()> {
        let (name, kind) = match item.base.0 {
            ast::ItemBase::Use(u) => {
                println!("Using {u:?}");
                return Ok(());
            }
            ast::ItemBase::Entity(e) => (e.name, EntryKind::Entity),
            ast::ItemBase::Arch(a) => {
                println!("Arch {a:?}");
                return Ok(());
            }
            ast::ItemBase::Record(r) => (r.name, EntryKind::Record),
            ast::ItemBase::Enum(e) => (e.name, EntryKind::Enum),
        };
        match scope.entries.entry(name.0) {
            hash_map::Entry::Occupied(entry) => {
                let original = &self.entry_pool[*entry.get()];
                self.messages.report(Message::already_in_scope(
                    name.0.get(),
                    name.1,
                    original.name.1,
                ))
            }
            hash_map::Entry::Vacant(entry) => {
                let idx = self.entry_pool.len();
                self.entry_pool.push(Entry { kind, name });
                entry.insert(idx);
            }
        }
        Ok(())
    }
}
