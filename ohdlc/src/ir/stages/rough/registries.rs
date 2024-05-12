use crate::{ast, ir::name_lookup::ScopeId};

#[derive(Debug)]
pub enum RoughType<'ast> {
    Record(ScopeId, &'ast ast::Record),
    Enum(&'ast ast::Enum),
}

#[derive(Debug)]
pub struct RoughEntity<'ast>(pub ScopeId, pub &'ast ast::Entity);

#[derive(Debug)]
pub struct RoughArch<'ast>(pub ScopeId, pub &'ast ast::Arch<'ast>);
