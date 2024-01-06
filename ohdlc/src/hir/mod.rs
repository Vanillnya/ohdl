mod modules;
mod resolving;
mod types;

use std::{collections::hash_map::Entry, fmt::Debug};

use crate::{message::Message, symbol::Ident, MESSAGES};

use self::{
    modules::Modules,
    resolving::{Resolvable, ResolvingScopes, ScopeId},
    types::Types,
};

pub mod stages;

#[derive(Debug)]
pub struct HIR<'hir> {
    pub types: Types<'hir>,
    pub modules: Modules,
    pub resolving_scopes: ResolvingScopes<'hir>,
}

impl<'hir> HIR<'hir> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            modules: Modules::default(),
            resolving_scopes: ResolvingScopes::new(),
        }
    }

    pub fn introduce(&mut self, scope: ScopeId, name: Ident, resolvable: Resolvable<'hir>) {
        match self.resolving_scopes[scope].entries.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert(resolvable);
            }
            Entry::Occupied(entry) => {
                let original = match *entry.get() {
                    Resolvable::Type(t) => self.types[t].name(),
                    Resolvable::Module(m) => self.modules[m].name,
                    Resolvable::Using(u) => *u.last().unwrap(),
                };
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }
}
