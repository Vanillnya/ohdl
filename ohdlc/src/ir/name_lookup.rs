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
    span::Span,
    symbol::{Ident, Symbol},
    MESSAGES,
};

use super::{import_bucket::ImportId, registry::Registry};

simple_key!(
    pub struct ScopeId;
);

pub type PreFlattenNameLookup = NameLookup<Resolvable>;
pub type PostFlattenNameLookup = NameLookup<Resolved>;

#[derive(Debug, Deref, DerefMut)]
pub struct NameLookup<L> {
    #[deref]
    pub scopes: SimpleSurotto<ScopeId, LookupScope<L>>,
    pub root: ScopeId,
}

impl<L> NameLookup<L> {
    pub fn new() -> Self {
        let mut scopes = SimpleSurotto::with_capacity(1);
        let root = scopes.insert(LookupScope {
            parent: None,
            entries: HashMap::new(),
        });
        Self { scopes, root }
    }

    pub fn sub_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.insert(LookupScope {
            parent: Some(parent),
            entries: HashMap::new(),
        })
    }

    pub fn lookup(&self, scope: ScopeId, lookup: &Ident, start: PathStart) -> Option<&L> {
        let mut scope = &self[scope];
        loop {
            match scope.entries.get(lookup) {
                None => match (scope.parent, start) {
                    (Some(p), PathStart::Indirect) => scope = &self[p],
                    _ => return None,
                },
                Some((_, l)) => return Some(l),
            }
        }
    }

    pub fn introduce(&mut self, scope: ScopeId, name: Ident, lookup: L) {
        match self[scope].entries.entry(name.0) {
            Entry::Vacant(entry) => {
                entry.insert((name.1, lookup));
            }
            Entry::Occupied(entry) => {
                let (original_span, _) = entry.get();
                MESSAGES.report(Message::already_in_scope(
                    name.0.get(),
                    name.1,
                    *original_span,
                ));
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
pub struct LookupScope<L> {
    pub parent: Option<ScopeId>,
    pub entries: HashMap<Symbol, (Span, L)>,
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
