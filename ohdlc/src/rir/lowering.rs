use std::collections::hash_map;

use bumpalo::Bump;

use crate::{
    ast,
    message::{Message, Messages},
    symbol::Ident,
};

use super::*;

pub type LResult<T> = Result<T, ()>;
pub type EntryIdx = usize;

pub struct RIR {
    arena: Bump,
    decl_pool: Vec<Declaration>,
    messages: &'static Messages,
}

impl RIR {
    pub fn new(messages: &'static Messages) -> Self {
        Self {
            arena: Bump::new(),
            decl_pool: Vec::new(),
            messages,
        }
    }

    pub fn lower_item(&mut self, scope: &mut Scope<'_>, item: ast::Item) -> LResult<()> {
        match item.base.0 {
            ast::ItemBase::Use(u) => {
                let imported_entry = u.path.0.last().unwrap().0;
                match scope.entries.entry(imported_entry.0) {
                    hash_map::Entry::Occupied(entry) => {
                        let original = match entry.get() {
                            Entry::Declared(idx) => (self.decl_pool[*idx]).name,
                            Entry::Imported(import) => import.segments.last().unwrap().0,
                        };
                        self.messages.report(Message::already_in_scope(
                            imported_entry.0.get(),
                            imported_entry.1,
                            original.1,
                        ));
                        return Err(()); // TODO: is this what we want?
                    }
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(Entry::Imported(Import {
                            segments: self.arena.alloc_slice_fill_iter(u.path.0.into_iter()),
                        }));
                    }
                }
                Ok(())
            }
            ast::ItemBase::Entity(e) => self.declare_entry(scope, e.name, DeclKind::Entity),
            ast::ItemBase::Arch(a) => {
                println!("Arch {a:?}");
                return Ok(());
            }
            ast::ItemBase::Record(r) => self.declare_entry(scope, r.name, DeclKind::Record),
            ast::ItemBase::Enum(e) => self.declare_entry(scope, e.name, DeclKind::Enum),
        }
    }

    fn declare_entry(&mut self, scope: &mut Scope, name: Ident, kind: DeclKind) -> LResult<()> {
        match scope.entries.entry(name.0) {
            hash_map::Entry::Occupied(entry) => {
                let original = match entry.get() {
                    Entry::Declared(idx) => (self.decl_pool[*idx]).name,
                    Entry::Imported(import) => import.segments.last().unwrap().0,
                };
                self.messages
                    .report(Message::already_in_scope(name.0.get(), name.1, original.1));
                return Err(()); // TODO: is this what we want?
            }
            hash_map::Entry::Vacant(entry) => {
                let idx = self.decl_pool.len();
                self.decl_pool.push(Declaration { kind, name });
                entry.insert(Entry::Declared(idx));
            }
        }
        Ok(())
    }
}
