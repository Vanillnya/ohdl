use crate::symbol::Ident;

#[derive(Debug)]
pub struct Declaration {
    pub kind: DeclKind,
    pub name: Ident,
}

#[derive(Debug)]
pub enum DeclKind {
    Module, // TODO: parsing
    Entity,
    Record,
    Enum,
}
