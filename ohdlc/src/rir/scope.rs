use std::collections::HashMap;

use super::{Decl, Ident};

#[derive(Debug)]
pub struct Scope<'r> {
    pub parent: Option<&'r Scope<'r>>,
    pub entries: HashMap<Ident, Decl>,
}
