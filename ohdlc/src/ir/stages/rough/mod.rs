use std::collections::{hash_map::Entry, VecDeque};

use crate::{
    ir::{
        modules::Modules,
        name_resolution::Import,
        resolving::{Resolvable, ResolvingScopes, ScopeId},
        types::Types,
    },
    message::Message,
    symbol::Ident,
    MESSAGES,
};

pub mod lowering;

#[derive(Debug)]
pub struct RoughIR<'ir> {
    pub types: Types<'ir>,
    pub modules: Modules,
    pub resolving_scopes: ResolvingScopes,
    pub imports: VecDeque<Import<'ir>>,
}

impl<'ir> RoughIR<'ir> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            modules: Modules::default(),
            resolving_scopes: ResolvingScopes::new(),
            imports: VecDeque::new(),
        }
    }

    pub fn introduce(&mut self, scope: ScopeId, name: Ident, resolvable: Resolvable) {
        match self.resolving_scopes[scope].entries.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert(resolvable);
            }
            Entry::Occupied(entry) => {
                let original = match *entry.get() {
                    Resolvable::Type(t) => self.types[t].name(),
                    Resolvable::Module(m) => self.modules[m].name,
                };
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }
}
