use bumpalo::Bump;

use crate::{
    ast,
    hir::{
        resolving::{Resolvable, ScopeId},
        HIR,
    },
    message::Message,
    span::Spanned,
    symbol::Ident,
    MESSAGES,
};

pub struct ResolveLowering<'a, 'hir> {
    pub arena: &'hir Bump,
    pub hir: &'a mut HIR<'hir>,
}

impl<'hir> ResolveLowering<'_, 'hir> {
    pub fn lower(&mut self, root: &[Spanned<ast::Item<'_>>]) {
        for item in root {
            self.lower_item(self.hir.resolving_scopes.root(), item);
        }
    }

    fn lower_item(&mut self, scope: ScopeId, item: &ast::Item<'_>) {
        match item {
            ast::Item::Use(u) => {
                if let Some(resolved) = self.resolve(
                    scope,
                    u.path.0.iter().map(|s| s.0).collect::<Vec<_>>().as_slice(),
                ) {
                    self.hir.resolving_scopes[scope]
                        .entries
                        .insert(u.path.0.last().unwrap().0 .0, resolved);
                }
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

    fn resolve(&mut self, mut scope: ScopeId, path: &[Ident]) -> Option<Resolvable<'hir>> {
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
        scope: ScopeId,
        resolvable: Option<Resolvable<'hir>>,
        segment: Ident,
    ) -> Option<Resolvable<'hir>> {
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
