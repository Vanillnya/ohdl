use deref_derive::{Deref, DerefMut};
use slab::Slab;
use std::fmt::Debug;

use crate::symbol::Ident;

#[derive(Default, Deref, DerefMut)]
pub struct Types<'hir>(Slab<Type<'hir>>);

impl Debug for Types<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(Debug)]
pub enum Type<'hir> {
    Entity(Entity),
    Record(Record),
    Enum(Enum<'hir>),
}

impl Type<'_> {
    pub fn id(&self) -> usize {
        match self {
            Type::Entity(e) => e.type_id,
            Type::Record(r) => r.type_id,
            Type::Enum(e) => e.type_id,
        }
    }

    pub fn name(&self) -> Ident {
        match self {
            Type::Entity(e) => e.name,
            Type::Record(r) => r.name,
            Type::Enum(e) => e.name,
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub type_id: usize,
    pub name: Ident,
    // TODO: ports
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
