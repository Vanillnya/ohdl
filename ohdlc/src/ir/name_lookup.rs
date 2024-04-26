use deref_derive::{Deref, DerefMut};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
};
use surotto::{simple::SimpleSurotto, simple_key};

use crate::{
    ast::PathStart,
    ir::{modules::ModuleId, types::TypeId},
    message::Message,
    symbol::{Ident, Symbol},
    MESSAGES,
};

use super::{
    name_resolution::{ImportId, ImportResult, NameResolution},
    registry::Registry,
};

simple_key!(
    pub struct ScopeId;
);

#[derive(Debug, Deref, DerefMut)]
pub struct NameLookup {
    #[deref]
    pub scopes: SimpleSurotto<ScopeId, ResolvingScope>,
    pub root: ScopeId,
}

impl NameLookup {
    pub fn new() -> Self {
        let mut scopes = SimpleSurotto::with_capacity(1);
        let root = scopes.insert(ResolvingScope {
            parent: None,
            entries: HashMap::new(),
        });
        Self { scopes, root }
    }

    pub fn sub_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.insert(ResolvingScope {
            parent: Some(parent),
            entries: HashMap::new(),
        })
    }

    pub fn find_resolvable(
        &self,
        scope: ScopeId,
        segment: &Ident,
        start: PathStart,
        id: ImportId,
    ) -> Option<&Resolvable> {
        let mut scope = &self[scope];
        loop {
            match scope.entries.get(segment) {
                Some(Resolvable::Import(i)) if *i == id => match (scope.parent, start) {
                    (Some(p), PathStart::Indirect) => scope = &self[p],
                    _ => return None,
                },
                None => match (scope.parent, start) {
                    (Some(p), PathStart::Indirect) => scope = &self[p],
                    _ => return None,
                },
                Some(resolvable) => return Some(resolvable),
            }
        }
    }

    pub fn introduce(
        &mut self,
        scope: ScopeId,
        name: Ident,
        resolvable: Resolvable,
        registry: &Registry<'_>,
        name_resolution: &NameResolution<'_>,
    ) {
        match self[scope].entries.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert(resolvable);
            }
            Entry::Occupied(entry) => {
                let original = match *entry.get() {
                    Resolvable::Resolved(r) => self.name_of_resolved(r, registry),
                    Resolvable::Import(i) => {
                        let import = &*name_resolution.imports[i].borrow();
                        match import {
                            ImportResult::InProgress(i) => *i.path.last().unwrap(),
                            ImportResult::Finished(r) => self.name_of_resolved(*r, registry),
                        }
                    }
                };
                MESSAGES.report(Message::already_in_scope(name.0.get(), name.1, original.1));
            }
        }
    }

    fn name_of_resolved(&self, resolved: Resolved, registry: &Registry<'_>) -> Ident {
        match resolved {
            Resolved::Type(t) => registry.types[t].name(),
            Resolved::Module(m) => registry.modules[m].name,
        }
    }
}

#[derive(Debug)]
pub struct ResolvingScope {
    pub parent: Option<ScopeId>,
    pub entries: HashMap<Symbol, Resolvable>,
}

#[derive(Debug, Clone, Copy)]
pub enum Resolvable {
    Resolved(Resolved),
    Import(ImportId),
}

#[derive(Debug, Clone, Copy)]
pub enum Resolved {
    Type(TypeId),
    Module(ModuleId),
}
