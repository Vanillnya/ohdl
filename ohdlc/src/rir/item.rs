use crate::span::Span;

use super::Ident;

#[derive(Debug)]
pub struct Decl {
    pub base_span: Span,
    pub base: DeclKind,
    pub name: Ident,
}

#[derive(Debug)]
pub enum DeclKind {
    Entity,
    Arch,
    Record,
    Enum,
}
