use crate::symbol::Ident;

#[derive(Debug)]
pub struct Entry {
    pub kind: EntryKind,
    pub name: Ident,
}

#[derive(Debug)]
pub enum EntryKind {
    Module, // TODO: parsing
    Entity,
    Record,
    Enum,
}
