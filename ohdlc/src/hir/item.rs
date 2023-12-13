use bumpalo::collections::Vec;
use bumpalo::Bump;

use crate::ast;
use crate::hir;

pub struct Item<'h> {
    pub base: hir::ItemBase<'h>,
}

impl<'h> hir::Item<'h> {
    pub fn represent(arena: &'h mut Bump, item: ast::Item) -> Self {
        Self {
            base: hir::ItemBase::represent(arena, item.base.0),
        }
    }
}

pub enum ItemBase<'h> {
    Record(hir::Record<'h>),
}

impl<'h> hir::ItemBase<'h> {
    pub fn represent(arena: &'h mut Bump, base: ast::ItemBase) -> Self {
        match base {
            ast::ItemBase::Record(record) => {
                hir::ItemBase::Record(hir::Record::represent(arena, record))
            }
            _ => todo!(),
        }
    }
}

pub struct Record<'h> {
    pub fields: Vec<'h, hir::Field>,
}

impl<'h> hir::Record<'h> {
    pub fn represent(arena: &'h mut Bump, record: ast::Record) -> Self {
        let fields = Vec::with_capacity_in(record.fields.len(), arena);
        for field in record.fields {}
        Self { fields }
    }
}

pub struct Field {}
