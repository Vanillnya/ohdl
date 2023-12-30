use std::fmt::Debug;

use slab::Slab;

use crate::symbol::Ident;

pub mod stages;

pub struct HIR<'hir> {
    pub types: Slab<Declaration<'hir>>,
}

impl HIR<'_> {
    pub fn new() -> Self {
        Self { types: Slab::new() }
    }
}

impl Debug for HIR<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.types.iter()).finish()
    }
}

#[derive(Debug)]
pub enum Declaration<'hir> {
    Enum(Enum<'hir>),
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
