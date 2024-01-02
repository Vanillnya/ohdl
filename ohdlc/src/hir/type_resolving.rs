use deref_derive::{Deref, DerefMut};
use slab::Slab;
use std::{collections::HashMap, fmt::Debug};

use crate::symbol::Symbol;

#[derive(Default, Deref, DerefMut)]
pub struct TRScopes(Slab<TypeResolvingScope>);

impl Debug for TRScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub struct TypeResolvingScope {
    pub parent: Option<usize>, // can we make a fork of slab with NonNull keys? :>
    pub types: HashMap<Symbol, usize>,
}
