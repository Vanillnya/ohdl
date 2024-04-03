use deref_derive::{Deref, DerefMut};
use std::{collections::HashMap, fmt::Debug};
use surotto::{simple::SimpleSurotto, simple_key};

use crate::{
    ast::PathStart,
    ir::{modules::ModuleId, types::TypeId},
    symbol::{Ident, Symbol},
};

use super::name_resolution::ImportId;

simple_key!(
    pub struct ScopeId;
);

#[derive(Debug, Deref, DerefMut)]
pub struct ResolvingScopes {
    #[deref]
    pub scopes: SimpleSurotto<ScopeId, ResolvingScope>,
    pub root: ScopeId,
}

impl ResolvingScopes {
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
    ) -> Option<&Resolvable> {
        let mut scope = &self[scope];
        loop {
            match scope.entries.get(segment) {
                Some(resolvable) => return Some(resolvable),
                None => match (scope.parent, start) {
                    (Some(p), PathStart::Indirect) => scope = &self[p],
                    _ => return None,
                },
            }
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
