use std::collections::HashMap;

use crate::{ast, symbol::Symbol};

use super::lowering::EntryIdx;

#[derive(Debug)]
pub struct Import {
    pub segments: Vec<ast::PathSegment>,
}

#[derive(Debug)]
pub enum Entry {
    Declared(EntryIdx),
    Imported(Import),
}

#[derive(Debug)]
pub struct Scope<'r> {
    pub parent: Option<&'r Scope<'r>>,
    pub entries: HashMap<Symbol, Entry>,
}
