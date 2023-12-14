use crate::span::Span;

use super::Ident;

#[derive(Debug)]
pub struct Item {
    pub base_span: Span,
    pub base: ItemBase,
}

#[derive(Debug)]
pub enum ItemBase {
    Entity(Ident),
    Arch(Ident),
    Record(Ident),
    Enum(Ident),
}
