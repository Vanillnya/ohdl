use deref_derive::{Deref, DerefMut};
use slab::Slab;
use std::{collections::HashMap, fmt::Debug};

use crate::symbol::Symbol;

#[derive(Default, Deref, DerefMut)]
pub struct ResolvingScopes(Slab<ResolvingScope>);

impl Debug for ResolvingScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub struct ResolvingScope {
    pub parent: Option<usize>, // can we make a fork of slab with NonNull keys? :>
    pub entries: HashMap<Symbol, Resolvable>,
}

#[derive(Debug)]
pub enum Resolvable {
    Type(usize),
    Module(usize),
}
