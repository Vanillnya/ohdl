use deref_derive::{Deref, DerefMut};
use std::fmt::Debug;
use surotto::{simple::SimpleSurotto, simple_key};

use crate::symbol::Ident;

use super::ScopeId;

simple_key!(
    pub struct ModuleId;
);

#[derive(Default, Deref, DerefMut)]
pub struct Modules(SimpleSurotto<ModuleId, Module>);

impl Debug for Modules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: Ident,
    pub scope: ScopeId,
}
