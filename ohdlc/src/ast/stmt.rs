use super::{
    expr::Expr,
    span::{Span, Spanned},
    ty::Type,
    Ident,
};

#[derive(Debug)]
pub enum Stmt<'s> {
    /// ```ohdl
    /// place MyEntity::MyArch {
    ///     i <= i,
    ///     o => o,
    /// }
    /// ```
    Place(PlaceStmt<'s>),
    /// ```ohdl
    /// val <= a or b;
    /// ```
    Assign(AssignStmt<'s>),
}

#[derive(Debug)]
pub struct PlaceStmt<'s> {
    pub entity_ty: Spanned<Type<'s>>,
    pub arch_ty: Spanned<Type<'s>>,
    pub links: Vec<Spanned<PlaceLink<'s>>>,
}

/// ```ohdl
/// src <= dst;
/// src => wire dst;
/// ```
#[derive(Debug)]
pub struct PlaceLink<'s> {
    pub src: Ident<'s>,
    pub arrow_span: Span,
    pub link: PlaceLinkInternal<'s>,
}

#[derive(Debug)]
pub enum PlaceLinkInternal<'s> {
    Ingoing(Spanned<Expr<'s>>),
    Outgoing(Spanned<Connector<'s>>),
}

#[derive(Debug)]
pub enum Connector<'s> {
    Ref(Ident<'s>),
    NewSignal(Ident<'s>),
}

#[derive(Debug)]
pub struct AssignStmt<'s> {
    pub assignee: Ident<'s>,
    pub value: Expr<'s>,
}
