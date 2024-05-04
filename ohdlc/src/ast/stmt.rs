use super::{expr::Expr, ty::Type};
use crate::{
    span::{Span, Spanned},
    symbol::Ident,
};

#[derive(Debug)]
pub enum Stmt<'ast> {
    /// ```ohdl
    /// MyEntity::MyArch {
    ///     i <= i,
    ///     o => o,
    /// }
    /// ```
    Place(PlaceStmt<'ast>),
    /// ```ohdl
    /// val <= a or b;
    /// ```
    Assign(AssignStmt<'ast>),
}

#[derive(Debug)]
pub struct PlaceStmt<'ast> {
    pub entity_ty: Spanned<Type>,
    pub arch_ty: Spanned<Type>,
    pub links: Vec<Spanned<PlaceLink<'ast>>>,
}

/// ```ohdl
/// src <= dst;
/// src => wire dst;
/// ```
#[derive(Debug)]
pub struct PlaceLink<'ast> {
    pub src: Ident,
    pub arrow_span: Span,
    pub link: PlaceLinkInternal<'ast>,
}

#[derive(Debug)]
pub enum PlaceLinkInternal<'ast> {
    Ingoing(Spanned<Expr<'ast>>),
    Outgoing(Spanned<Connector>),
}

#[derive(Debug)]
pub enum Connector {
    Ref(Ident),
    NewSignal(Ident),
}

#[derive(Debug)]
pub struct AssignStmt<'ast> {
    pub assignee: Ident,
    pub value: Expr<'ast>,
}
