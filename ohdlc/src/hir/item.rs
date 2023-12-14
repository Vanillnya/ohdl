use bumpalo::collections::Vec;
use bumpalo::Bump;

use crate::ast;
use crate::hir;
use crate::span::Span;

pub struct Item<'h> {
    pub base_span: Span,
    pub base: hir::ItemBase<'h>,
}

impl<'h> hir::Item<'h> {
    pub fn represent(arena: &'h Bump, item: ast::Item) -> Self {
        Self {
            base_span: item.base.1,
            base: hir::ItemBase::represent(arena, item.base.0),
        }
    }
}

pub enum ItemBase<'h> {
    Record(hir::Record<'h>),
}

impl<'h> hir::ItemBase<'h> {
    pub fn represent(arena: &'h Bump, base: ast::ItemBase) -> Self {
        match base {
            ast::ItemBase::Record(record) => {
                hir::ItemBase::Record(hir::Record::represent(arena, record))
            }
            _ => todo!(),
        }
    }
}

pub struct Record<'h> {
    pub name: hir::Ident,
    pub fields: Vec<'h, hir::Field>,
}

impl<'h> hir::Record<'h> {
    pub fn represent(arena: &'h Bump, record: ast::Record) -> Self {
        let mut fields = Vec::with_capacity_in(record.fields.len(), arena);
        for field in record.fields {
            fields.push(hir::Field::represent(arena, field.0));
        }
        Self {
            name: hir::Ident::intern(record.name.0),
            fields,
        }
    }
}

pub struct Field {
    pub name: hir::Ident,
}

impl hir::Field {
    pub fn represent(_arena: &Bump, field: ast::Field) -> Self {
        Self {
            name: hir::Ident::intern(field.name.0),
        }
    }
}
