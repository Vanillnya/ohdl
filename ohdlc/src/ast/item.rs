use crate::{span::Spanned, symbol::Ident};

use super::{
    stmt::Stmt,
    ty::{Path, Type},
};

#[derive(Debug)]
pub enum Item<'a> {
    Use(Use),
    Module(Module<'a>),
    Entity(Entity),
    Arch(Arch<'a>),
    Record(Record),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Use {
    pub path: Path,
}

#[derive(Debug)]
pub struct Module<'a> {
    pub name: Ident,
    pub items: Vec<Spanned<Item<'a>>>,
}

#[derive(Debug)]
pub struct Entity {
    pub name: Ident,
    pub ports: Vec<Spanned<Port>>,
}

#[derive(Debug)]
pub struct Port {
    pub kind: Spanned<PortKind>,
    pub name: Ident,
    pub ty: Spanned<Type>,
}

#[derive(Debug)]
pub enum PortKind {
    Input,
    Output,
    // TODO: inout
}

#[derive(Debug)]
pub struct Arch<'a> {
    pub name: Ident,
    pub ty: Spanned<Type>,
    pub stmts: Vec<Spanned<Stmt<'a>>>,
}

#[derive(Debug)]
pub struct Record {
    pub name: Ident,
    pub fields: Vec<Spanned<Field>>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub ty: Spanned<Type>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<Ident>,
}
