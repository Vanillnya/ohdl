use crate::ast;

#[derive(Debug)]
pub enum RoughType<'ast> {
    Entity(&'ast ast::Entity),
    Record(&'ast ast::Record),
    Enum(&'ast ast::Enum),
}
