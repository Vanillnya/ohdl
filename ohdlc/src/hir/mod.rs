mod modules;
mod resolving;
mod types;

use std::fmt::Debug;

use self::{modules::Modules, resolving::ResolvingScopes, types::Types};

pub mod stages;

#[derive(Debug)]
pub struct HIR<'hir> {
    pub types: Types<'hir>,
    pub modules: Modules,
    pub resolving_scopes: ResolvingScopes,
}

impl HIR<'_> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            modules: Modules::default(),
            resolving_scopes: ResolvingScopes::default(),
        }
    }
}
