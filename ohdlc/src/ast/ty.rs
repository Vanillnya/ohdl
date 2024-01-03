use crate::{span::Spanned, symbol::Ident};

#[derive(Debug)]
pub struct Path(pub Vec<PathSegment>);

#[derive(Debug, Clone, Copy)]
pub struct PathSegment(pub Ident);

#[derive(Debug)]
pub struct Type {
    pub path: Spanned<Path>,
}
