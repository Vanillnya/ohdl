use crate::{span::Spanned, symbol::Ident};

use super::{
    stmt::Stmt,
    ty::{Path, Type},
};

#[derive(Debug)]
pub enum Item<'ast> {
    Use(Use),
    Module(Module<'ast>),
    Entity(Entity),
    Arch(Arch<'ast>),
    Record(Record),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Use {
    pub path: Spanned<Path>,
}

#[derive(Debug)]
pub struct Module<'ast> {
    pub name: Ident,
    pub items: Vec<Spanned<Item<'ast>>>,
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

#[derive(Debug, Clone, Copy)]
pub enum PortKind {
    Input,
    Output,
    // TODO: inout
}

#[derive(Debug)]
pub struct Arch<'ast> {
    pub name: Ident,
    pub ty: Spanned<Type>,
    pub stmts: Vec<Spanned<Stmt<'ast>>>,
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
