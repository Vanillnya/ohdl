mod type_resolving;
mod types;

use std::fmt::Debug;

use self::{type_resolving::TRScopes, types::Types};

pub mod stages;

#[derive(Debug)]
pub struct HIR<'hir> {
    pub types: Types<'hir>,
    pub tr_scopes: TRScopes,
}

impl HIR<'_> {
    pub fn new() -> Self {
        Self {
            types: Types::default(),
            tr_scopes: TRScopes::default(),
        }
    }
}
