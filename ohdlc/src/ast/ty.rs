use crate::span::Spanned;

use super::Ident;

#[derive(Debug)]
pub struct Path<'s>(pub Vec<PathSegment<'s>>);

#[derive(Debug)]
pub struct PathSegment<'s>(pub Ident<'s>);

#[derive(Debug)]
pub struct Type<'s> {
    pub path: Spanned<Path<'s>>,
}
