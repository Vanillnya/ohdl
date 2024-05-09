use crate::{span::Spanned, symbol::Ident};

#[derive(Debug)]
pub struct Path(pub Vec<PathSegment>, pub PathStart);

#[derive(Debug, Clone, Copy)]
pub enum PathStart {
    /// Search in the root scope
    /// e.g. `::path::path`
    Root,
    /// Search in the current scope and upwards
    /// e.g. `path::path`
    Local,
}

#[derive(Debug, Clone, Copy)]
pub struct PathSegment(pub Ident);

#[derive(Debug)]
pub struct Type {
    pub path: Spanned<Path>,
}
