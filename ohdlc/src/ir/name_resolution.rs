use std::collections::VecDeque;

use surotto::{simple::SimpleSurotto, simple_key};

use crate::{ast::PathStart, symbol::Ident};

use super::resolving::{Resolved, ScopeId};

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
}

#[derive(Debug)]
pub enum ImportResult<'ir> {
    InProgress(Import<'ir>),
    Finished(Resolved),
}

simple_key!(
    pub struct ImportId;
);

#[derive(Debug)]
pub struct NameResolution<'ir> {
    pub imports: SimpleSurotto<ImportId, ImportResult<'ir>>,
    pub queue: VecDeque<ImportId>,
}

impl<'ir> NameResolution<'ir> {
    pub fn new() -> Self {
        Self {
            imports: SimpleSurotto::new(),
            queue: VecDeque::new(),
        }
    }
}
