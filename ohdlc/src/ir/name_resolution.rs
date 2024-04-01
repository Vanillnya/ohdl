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
