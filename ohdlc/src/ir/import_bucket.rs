use surotto::{simple::SimpleSurotto, simple_assoc::SimpleAssocSurotto, simple_key};

use crate::{span::Span, symbol::Ident};

use super::name_lookup::ScopeId;

simple_key!(
    pub struct ImportId;
);

#[derive(Debug)]
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
///     use path::path::path;
///         ^^^^  ~~~~  ~~~~
///          |     \direct/
///       indirect
/// }
/// ```
#[derive(Debug)]
pub struct Import<'ir> {
    /// The scope we take the path from
    pub scope: ScopeId,
    /// How the lookup of the first segment should be handled.
    pub strategy: LookupStrategy,
    /// The path we have to take from the source
    pub path: &'ir [Ident],
    /// The whole import span
    pub span: Span,
    /// The scope the import will be added to
    pub target_scope: ScopeId,
}

#[derive(Debug, Clone, Copy)]
pub enum LookupStrategy {
    /// Look only in the given scope.
    Direct,
    /// Look in the given scope and upwards.
    Indirect,
}
