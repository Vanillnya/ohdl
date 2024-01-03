use bumpalo::Bump;

use crate::{
    ast,
    hir::{resolving::Resolvable, HIR},
    message::Message,
    span::Spanned,
    symbol::{Ident, Symbol},
    MESSAGES,
};

pub struct ResolveLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> ResolveLowering<'_, 'hir> {
    pub fn lower(&mut self, root: &[Spanned<ast::Item>]) {
        let root_scope = 0;
        for item in root {
            self.lower_item(root_scope, item);
        }
    }

    fn lower_item(&mut self, scope: usize, item: &ast::Item) {
        match item {
            ast::Item::Use(u) => {
                let mut search_scope = scope;
                let mut segments = u.path.0.iter();
            }
            ast::Item::Module(m) => {
                let sub_scope = match self.hir.resolving_scopes[scope].entries[&m.name.0] {
                    Resolvable::Module(m) => self.hir.modules[m].scope,
                    Resolvable::Type(_) | Resolvable::Using(_) => panic!("whut"),
                };
                for i in &m.items {
                    self.lower_item(sub_scope, i);
                }
            }
            _ => {}
        }
    }

    fn resolve(&mut self, mut scope: usize, path: &'hir [Ident]) -> Option<Resolvable> {
        for segment in path.iter().take(path.len() - 1) {
            let resolvable = self.hir.resolving_scopes.find(scope, segment.0);
            let resolvable = self.unroll(scope, resolvable, *segment)?;
            match resolvable {
                Resolvable::Module(m) => {
                    scope = self.hir.modules[m].scope;
                }
                Resolvable::Type(_) => {
                    MESSAGES.report(Message::use_continues_after_type(segment.1));
                }
                Resolvable::Using(_) => unreachable!(),
            }
        }

        let segment = path.last().unwrap();
        let resolvable = self.hir.resolving_scopes.find(scope, segment.0);
        let resolvable = self.unroll(scope, resolvable, *segment)?;
        Some(resolvable)
    }

    fn unroll(
        &mut self,
        scope: usize,
        resolvable: Option<Resolvable<'hir>>,
        segment: Ident,
    ) -> Option<Resolvable> {
        match resolvable {
            Some(Resolvable::Using(path)) => {
                let resolved = self.resolve(scope, path)?; // TODO: how do we prevent multi error?
                self.hir.resolving_scopes[scope]
                    .entries
                    .insert(segment.0, resolved);
                Some(resolved)
            }
            Some(r) => Some(r),
            None => {
                MESSAGES.report(Message::could_not_resolve(segment));
                None
            }
        }
    }
}
