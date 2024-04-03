use std::collections::hash_map::Entry;

use crate::{message::Message, symbol::Ident, MESSAGES};

use self::{
    modules::Modules,
    name_resolution::{ImportResult, NameResolution},
    resolving::{Resolvable, ResolvingScopes, ScopeId},
    types::Types,
};

mod modules;
mod name_resolution;
mod resolving;
mod types;

pub mod stages;

#[derive(Debug)]
pub struct IR<'ir> {
    pub types: Types<'ir>,
    pub modules: Modules,
    pub resolving_scopes: ResolvingScopes,
    pub name_resolution: NameResolution<'ir>,
}

impl<'ir> IR<'ir> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            modules: Modules::default(),
            resolving_scopes: ResolvingScopes::new(),
            name_resolution: NameResolution::new(),
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
                    Resolvable::Import(i) => {
                        let import = &self.name_resolution.imports[i];
                        match import {
                            ImportResult::InProgress(i) => *i.path.last().unwrap(),
                            ImportResult::Finished(_) => {
                                // TODO: can this happen? :3
                                panic!("Lixou has to think about this - can this even happen?")
                            }
                        }
                    }
                };
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }
}
