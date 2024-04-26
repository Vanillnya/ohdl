use std::fmt::Debug;
use surotto::simple_key;

use crate::symbol::Ident;

use super::name_lookup::ScopeId;

simple_key!(
    pub struct ModuleId;
);

#[derive(Debug)]
pub struct Module {
    pub name: Ident,
    pub scope: ScopeId,
}
