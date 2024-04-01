use deref_derive::{Deref, DerefMut};
use std::{collections::HashMap, fmt::Debug};
use surotto::simple::SimpleSurotto;

use crate::{
    ir::{modules::ModuleId, types::TypeId, ScopeId},
    symbol::Symbol,
};

#[derive(Deref, DerefMut)]
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
}

impl Debug for ResolvingScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.scopes.iter()).finish()
    }
}

#[derive(Debug)]
pub struct ResolvingScope {
    pub parent: Option<ScopeId>,
    pub entries: HashMap<Symbol, Resolvable>,
}

#[derive(Debug, Clone, Copy)]
pub enum Resolvable {
    Type(TypeId),
    Module(ModuleId),
}
