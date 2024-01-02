use deref_derive::{Deref, DerefMut};
use slab::Slab;
use std::fmt::Debug;

use crate::symbol::Ident;

#[derive(Default, Deref, DerefMut)]
pub struct Modules(Slab<Module>);

impl Debug for Modules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: Ident,
    pub scope: usize,
}
