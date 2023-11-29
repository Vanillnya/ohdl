use super::{expr::Expr, item::PortKind, span::Spanned, ty::Type, Ident};

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
    /// Name of wire on entity side
    pub src: Ident<'s>,
    pub kind: Spanned<PortKind>,
    pub dst: LinkDest<'s>,
}

#[derive(Debug)]
pub enum LinkDest<'s> {
    Ref(Ident<'s>),
    NewWire(Ident<'s>),
}

#[derive(Debug)]
pub struct AssignStmt<'s> {
    pub assignee: Ident<'s>,
    pub value: Expr<'s>,
}
