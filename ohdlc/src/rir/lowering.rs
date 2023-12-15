use bumpalo::Bump;

use crate::ast;

use super::*;

pub struct RIR {
    arena: Bump,
}

impl RIR {
    pub fn new() -> Self {
        Self { arena: Bump::new() }
    }

    pub fn lower_item(&self, scope: &mut Scope, item: ast::Item) {
        match item.base.0 {
            ast::ItemBase::Use(u) => {
                println!("Using {u:?}");
            }
            ast::ItemBase::Entity(e) => {
                scope.entries.insert(
                    Ident::intern(e.name.0),
                    Decl {
                        base_span: item.base.1,
                        base: DeclKind::Entity,
                        name: Ident::intern(e.name.0),
                    },
                );
            }
            ast::ItemBase::Arch(a) => {
                scope.entries.insert(
                    Ident::intern(a.name.0),
                    Decl {
                        base_span: item.base.1,
                        base: DeclKind::Arch,
                        name: Ident::intern(a.name.0),
                    },
                );
            }
            ast::ItemBase::Record(r) => {
                scope.entries.insert(
                    Ident::intern(r.name.0),
                    Decl {
                        base_span: item.base.1,
                        base: DeclKind::Record,
                        name: Ident::intern(r.name.0),
                    },
                );
            }
            ast::ItemBase::Enum(e) => {
                scope.entries.insert(
                    Ident::intern(e.name.0),
                    Decl {
                        base_span: item.base.1,
                        base: DeclKind::Enum,
                        name: Ident::intern(e.name.0),
                    },
                );
            }
        };
    }
}
