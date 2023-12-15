use std::collections::HashMap;

use crate::symbol::Symbol;

use super::Decl;

#[derive(Debug)]
pub struct Scope<'r> {
    pub parent: Option<&'r Scope<'r>>,
    pub entries: HashMap<Symbol, Decl>,
}
