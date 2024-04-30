use petgraph::{graph::NodeIndex, Graph};

use crate::{ast::PathStart, span::Span, symbol::Ident};

use super::name_lookup::ScopeId;

pub struct ImportBucket<'ir> {
    graph: Graph<Import<'ir>, ()>,
}

impl<'ir> ImportBucket<'ir> {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn insert(&mut self, import: Import<'ir>) -> ImportId {
        ImportId(self.graph.add_node(import))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImportId(NodeIndex<u32>);

/// ```ohdl,ignore
/// mod scope {
///     // indirect
///     use path::path::path;
///     // direct
///     use ::path::path::path;
/// }
/// ```
#[derive(Debug)]
pub struct Import<'ir> {
    /// The scope we take the path from
    pub scope: ScopeId,
    /// If the path starts directly or indirectly.
    pub start: PathStart,
    /// The path we have to take from the source
    pub path: &'ir [Ident],
    /// The whole import span
    pub span: Span,
}
