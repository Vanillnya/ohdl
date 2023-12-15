use super::{expr::Expr, ty::Type};
use crate::{
    span::{Span, Spanned},
    symbol::Ident,
};

#[derive(Debug)]
pub enum Stmt<'a> {
    /// ```ohdl
    /// place MyEntity::MyArch {
    ///     i <= i,
    ///     o => o,
    /// }
    /// ```
    Place(PlaceStmt<'a>),
    /// ```ohdl
    /// val <= a or b;
    /// ```
    Assign(AssignStmt<'a>),
}

#[derive(Debug)]
pub struct PlaceStmt<'a> {
    pub entity_ty: Spanned<Type>,
    pub arch_ty: Spanned<Type>,
    pub links: Vec<Spanned<PlaceLink<'a>>>,
}

/// ```ohdl
/// src <= dst;
/// src => wire dst;
/// ```
#[derive(Debug)]
pub struct PlaceLink<'a> {
    pub src: Ident,
    pub arrow_span: Span,
    pub link: PlaceLinkInternal<'a>,
}

#[derive(Debug)]
pub enum PlaceLinkInternal<'a> {
    Ingoing(Spanned<Expr<'a>>),
    Outgoing(Spanned<Connector>),
}

#[derive(Debug)]
pub enum Connector {
    Ref(Ident),
    NewSignal(Ident),
}

#[derive(Debug)]
pub struct AssignStmt<'a> {
    pub assignee: Ident,
    pub value: Expr<'a>,
}
