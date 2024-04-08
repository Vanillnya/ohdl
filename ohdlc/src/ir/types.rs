use deref_derive::{Deref, DerefMut};
use std::fmt::Debug;
use surotto::{simple::SimpleSurotto, simple_key};

use crate::{ast::PortKind, symbol::Ident};

use super::name_resolution::ImportId;

simple_key!(
    pub struct TypeId;
);

#[derive(Default, Deref, DerefMut)]
pub struct Types<'hir>(SimpleSurotto<TypeId, Type<'hir>>);

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
    pub fn id(&self) -> TypeId {
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
    pub type_id: TypeId,
    pub name: Ident,
    pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct Port {
    // TODO: we should agree on whether to use ast types in ir or ir types in ast, not both.
    pub kind: PortKind,
    pub name: Ident,
    pub ty: ImportId,
}

#[derive(Debug)]
pub struct Record {
    pub type_id: TypeId,
    pub name: Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub ty: ImportId,
}

#[derive(Debug)]
pub struct Enum<'hir> {
    pub type_id: TypeId,
    pub name: Ident,
    pub variants: &'hir [Variant],
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
}
