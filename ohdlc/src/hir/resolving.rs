use deref_derive::{Deref, DerefMut};
use slab::Slab;
use std::{collections::HashMap, fmt::Debug};

use crate::symbol::{Ident, Symbol};

#[derive(Default, Deref, DerefMut)]
pub struct ResolvingScopes<'hir>(Slab<ResolvingScope<'hir>>);

impl<'hir> ResolvingScopes<'hir> {
    pub fn sub_scope(&mut self, parent: usize) -> usize {
        self.0.insert(ResolvingScope {
            parent: Some(parent),
            entries: HashMap::new(),
        })
    }

    pub fn find(&self, scope: usize, symbol: Symbol) -> Option<Resolvable<'hir>> {
        let mut s = &self[scope];

        loop {
            match s.entries.get(&symbol) {
                Some(r) => return Some(*r),
                None => match s.parent {
                    Some(p) => {
                        s = &self[p];
                    }
                    None => return None,
                },
            }
        }
    }
}

impl Debug for ResolvingScopes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub struct ResolvingScope<'hir> {
    pub parent: Option<usize>, // can we make a fork of slab with NonNull keys? :>
    pub entries: HashMap<Symbol, Resolvable<'hir>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Resolvable<'hir> {
    Type(usize),
    Module(usize),
    Using(&'hir [Ident]),
}
