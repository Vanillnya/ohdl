use std::{collections::HashMap, fmt::Debug};

use slab::Slab;

use crate::symbol::{Ident, Symbol};

pub mod stages;

pub struct HIR<'hir> {
    pub types: Slab<Declaration<'hir>>,
    pub tr_scopes: Slab<TypeResolvingScope>,
}

impl HIR<'_> {
    pub fn new() -> Self {
        Self {
            types: Slab::new(),
            tr_scopes: Slab::new(),
        }
    }
}

impl Debug for HIR<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.types.iter()).finish()
    }
}

pub struct TypeResolvingScope {
    pub parent: Option<usize>, // can we make a fork of slab with NonNull keys? :>
    pub types: HashMap<Symbol, usize>,
}

#[derive(Debug)]
pub enum Declaration<'hir> {
    Record(Record),
    Enum(Enum<'hir>),
}

impl Declaration<'_> {
    pub fn id(&self) -> usize {
        match self {
            Declaration::Record(r) => r.type_id,
            Declaration::Enum(e) => e.type_id,
        }
    }

    pub fn name(&self) -> Ident {
        match self {
            Declaration::Record(r) => r.name,
            Declaration::Enum(e) => e.name,
        }
    }
}

#[derive(Debug)]
pub struct Record {
    pub type_id: usize,
    pub name: Ident,
    // TODO: fields
}

#[derive(Debug)]
pub struct Enum<'hir> {
    pub type_id: usize,
    pub name: Ident,
    pub variants: &'hir [Variant],
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
}
