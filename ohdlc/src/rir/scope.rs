use std::collections::HashMap;

use crate::{ast, symbol::Symbol};

use super::lowering::EntryIdx;

#[derive(Debug)]
pub struct Import<'r> {
    pub segments: &'r [ast::PathSegment],
}

#[derive(Debug)]
pub enum Entry<'r> {
    Declared(EntryIdx),
    Imported(Import<'r>),
}

#[derive(Debug)]
pub struct Scope<'r> {
    pub parent: Option<&'r Scope<'r>>,
    pub entries: HashMap<Symbol, Entry<'r>>,
}
