mod resolving;
mod types;

use std::fmt::Debug;

use self::{resolving::ResolvingScopes, types::Types};

pub mod stages;

#[derive(Debug)]
pub struct HIR<'hir> {
    pub types: Types<'hir>,
    pub tr_scopes: ResolvingScopes,
}

impl HIR<'_> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            tr_scopes: ResolvingScopes::default(),
        }
    }
}
