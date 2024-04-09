use crate::{span::Spanned, symbol::Ident};

#[derive(Debug)]
pub struct Path(pub Vec<PathSegment>, pub PathStart);

// TODO: don't we want this to be (or also be) root? maybe?
#[derive(Debug, Clone, Copy)]
pub enum PathStart {
    /// Search directly in the given scope for the next path segment
    /// e.g. `::path::path`
    Direct,
    /// e.g. `path::path`
    Indirect,
}

#[derive(Debug, Clone, Copy)]
pub struct PathSegment(pub Ident);

#[derive(Debug)]
pub struct Type {
    pub path: Spanned<Path>,
}
