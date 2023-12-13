use crate::span::Spanned;

use super::{
    stmt::Stmt,
    ty::{Path, Type},
    Ident,
};

#[derive(Debug)]
pub struct Item<'s> {
    pub base: Spanned<ItemBase<'s>>,
}

#[derive(Debug)]
pub enum ItemBase<'s> {
    Use(Use<'s>),
    Entity(Entity<'s>),
    Arch(Arch<'s>),
    Record(Record<'s>),
    Enum(Enum<'s>),
}

#[derive(Debug)]
pub struct Use<'s> {
    pub path: Path<'s>,
}

#[derive(Debug)]
pub struct Entity<'s> {
    pub name: Ident<'s>,
    pub ports: Vec<Spanned<Port<'s>>>,
}

#[derive(Debug)]
pub struct Port<'s> {
    pub kind: Spanned<PortKind>,
    pub name: Ident<'s>,
    pub ty: Spanned<Type<'s>>,
}

#[derive(Debug)]
pub enum PortKind {
    Input,
    Output,
    // TODO: inout
}

#[derive(Debug)]
pub struct Arch<'s> {
    pub name: Ident<'s>,
    pub ty: Spanned<Type<'s>>,
    pub stmts: Vec<Spanned<Stmt<'s>>>,
}

#[derive(Debug)]
pub struct Record<'s> {
    pub name: Ident<'s>,
    pub fields: Vec<Spanned<Field<'s>>>,
}

#[derive(Debug)]
pub struct Field<'s> {
    pub name: Ident<'s>,
    pub ty: Spanned<Type<'s>>,
}

#[derive(Debug)]
pub struct Enum<'s> {
    pub name: Ident<'s>,
    pub variants: Vec<Ident<'s>>,
}
