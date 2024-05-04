use crate::{ast::PortKind, ir::registry::TypeId, symbol::Ident};

#[derive(Debug)]
pub enum RefinedType<'hir> {
    Entity(Entity),
    Record(Record),
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
    pub ty: (),
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
    pub ty: (),
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
