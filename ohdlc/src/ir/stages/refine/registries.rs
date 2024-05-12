use crate::{
    ast::PortKind,
    ir::registries::{EntityId, TypeId},
    symbol::Ident,
};

#[derive(Debug)]
pub enum Type<'ir> {
    Record(Record<'ir>),
    Enum(Enum<'ir>),
}

#[derive(Debug)]
pub struct Entity<'ir> {
    pub type_id: EntityId,
    pub name: Ident,
    pub ports: &'ir [Port],
}

#[derive(Debug)]
pub struct Port {
    // TODO: we should agree on whether to use ast types in ir or ir types in ast, not both.
    pub kind: PortKind,
    pub name: Ident,
    pub ty: Option<TypeId>,
}

#[derive(Debug)]
pub struct Record<'ir> {
    pub type_id: TypeId,
    pub name: Ident,
    pub fields: &'ir [Field],
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub ty: Option<TypeId>,
}

#[derive(Debug)]
pub struct Enum<'ir> {
    pub type_id: TypeId,
    pub name: Ident,
    pub variants: &'ir [Variant],
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
}
