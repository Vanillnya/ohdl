use std::marker::PhantomData;

use super::{
    span::Spanned,
    ty::{Path, Type},
    Ident,
};

#[derive(Debug)]
pub struct Item<'s> {
    pub base: Spanned<ItemBase<'s>>,
}

#[derive(Debug)]
pub enum ItemBase<'s> {
    Entity(Entity<'s>),
    Arch(Arch<'s>),
    Use(Use<'s>),
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
    pub r#type: Spanned<Type<'s>>,
}

#[derive(Debug)]
pub enum PortKind {
    Input,
    Output,
    // TODO: inout
}

#[derive(Debug)]
pub struct Arch<'s> {
    pub _phantom: PhantomData<&'s ()>,
}

#[derive(Debug)]
pub struct Use<'s> {
    pub path: Path<'s>,
}
