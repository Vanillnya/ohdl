use crate::{ast::PortKind, ir::registry::TypeId, symbol::Ident};

#[derive(Debug)]
pub enum RefinedType<'hir> {
    Entity(Entity<'hir>),
    Record(Record<'hir>),
    Enum(Enum<'hir>),
}

impl RefinedType<'_> {
    pub fn id(&self) -> TypeId {
        match self {
            RefinedType::Entity(e) => e.type_id,
            RefinedType::Record(r) => r.type_id,
            RefinedType::Enum(e) => e.type_id,
        }
    }

    pub fn name(&self) -> Ident {
        match self {
            RefinedType::Entity(e) => e.name,
            RefinedType::Record(r) => r.name,
            RefinedType::Enum(e) => e.name,
        }
    }
}

#[derive(Debug)]
pub struct Entity<'hir> {
    pub type_id: TypeId,
    pub name: Ident,
    pub ports: &'hir [Port],
}

#[derive(Debug)]
pub struct Port {
    // TODO: we should agree on whether to use ast types in ir or ir types in ast, not both.
    pub kind: PortKind,
    pub name: Ident,
    pub ty: Option<TypeId>,
}

#[derive(Debug)]
pub struct Record<'hir> {
    pub type_id: TypeId,
    pub name: Ident,
    pub fields: &'hir [Field],
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub ty: Option<TypeId>,
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
