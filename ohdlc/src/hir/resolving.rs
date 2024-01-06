use deref_derive::{Deref, DerefMut};
use std::{collections::HashMap, fmt::Debug};
use surotto::{simple::SimpleSurotto, simple_key};

use crate::symbol::{Ident, Symbol};

use super::{modules::ModuleId, types::TypeId};

simple_key!(
    pub struct ScopeId;
);

#[derive(Deref, DerefMut)]
pub struct ResolvingScopes<'hir> {
    #[deref]
    scopes: SimpleSurotto<ScopeId, ResolvingScope<'hir>>,
    root: ScopeId,
}

impl<'hir> ResolvingScopes<'hir> {
    pub fn new() -> Self {
        let mut scopes = SimpleSurotto::with_capacity(1);
        let root = scopes.insert(ResolvingScope {
            parent: None,
            entries: HashMap::new(),
        });
        Self { scopes, root }
    }

    pub fn root(&self) -> ScopeId {
        self.root
    }
}

impl<'hir> ResolvingScopes<'hir> {
    pub fn sub_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.insert(ResolvingScope {
            parent: Some(parent),
            entries: HashMap::new(),
        })
    }

    pub fn find(&self, scope: ScopeId, symbol: Symbol) -> Option<Resolvable<'hir>> {
        let mut s = self.get(scope);

        loop {
            match s.entries.get(&symbol) {
                Some(r) => return Some(*r),
                None => match s.parent {
                    Some(p) => {
                        s = self.get(p);
                    }
                    None => return None,
                },
            }
        }
    }
}

impl Debug for ResolvingScopes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.scopes.iter()).finish()
    }
}

#[derive(Debug)]
pub struct ResolvingScope<'hir> {
    pub parent: Option<ScopeId>,
    pub entries: HashMap<Symbol, Resolvable<'hir>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Resolvable<'hir> {
    Type(TypeId),
    Module(ModuleId),
    Using(&'hir [Ident]),
}
