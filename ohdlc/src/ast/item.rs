use std::marker::PhantomData;

use super::{span::Spanned, Ident};

#[derive(Debug)]
pub struct Item<'s> {
    pub kind: ItemKind<'s>,
}

#[derive(Debug)]
pub enum ItemKind<'s> {
    Entity(Entity<'s>),
    Arch(Arch<'s>),
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
    pub r#type: Ident<'s>,
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
