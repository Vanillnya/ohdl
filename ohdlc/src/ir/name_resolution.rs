use std::collections::VecDeque;

use surotto::{simple::SimpleSurotto, simple_key};

use crate::symbol::Ident;

use super::resolving::ScopeId;

/// ```ohdl,ignore
/// mod src {
///     use path::path::path;
/// }
/// ```
#[derive(Debug)]
pub struct Import<'ir> {
    /// The scope we start from
    pub src: ScopeId,
    /// The path we have to take from the source
    pub path: &'ir [Ident],
}

simple_key!(
    pub struct ImportId;
);

#[derive(Debug)]
pub struct NameResolution<'ir> {
    pub imports: SimpleSurotto<ImportId, Import<'ir>>,
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
