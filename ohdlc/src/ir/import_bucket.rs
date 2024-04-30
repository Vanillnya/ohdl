use surotto::{simple::SimpleSurotto, simple_assoc::SimpleAssocSurotto, simple_key};

use crate::{ast::PathStart, span::Span, symbol::Ident};

use super::name_lookup::ScopeId;

simple_key!(
    pub struct ImportId;
);

pub struct ImportBucket<'ir> {
    pub imports: SimpleSurotto<ImportId, Import<'ir>>,
    pub dependants: SimpleAssocSurotto<ImportId, Vec<ImportId>>,
}

impl<'ir> ImportBucket<'ir> {
    pub fn new() -> Self {
        Self {
            imports: SimpleSurotto::new(),
            dependants: SimpleAssocSurotto::new(),
        }
    }

    pub fn insert(&mut self, import: Import<'ir>) -> ImportId {
        let id = self.imports.insert(import);
        self.dependants.insert(id, Vec::new());
        id
    }
}

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
