use crate::{ast, ir::name_lookup::ScopeId};

#[derive(Debug)]
pub struct RoughType<'ast>(pub ScopeId, pub RoughTypeItem<'ast>);

#[derive(Debug)]
pub enum RoughTypeItem<'ast> {
    Entity(&'ast ast::Entity),
    Record(&'ast ast::Record),
    Enum(&'ast ast::Enum),
}
