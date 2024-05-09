use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        import_bucket::{Import, ImportBucket, LookupStrategy},
        modules::Module,
        name_lookup::{PreFlattenNameLookup, Resolvable, Resolved, ScopeId},
        registry::Registry,
    },
    span::Spanned,
    symbol::Ident,
};

use self::types::{RoughType, RoughTypeItem};

pub mod types;

pub struct RoughStage<'ir, 'b, 'ast> {
    pub arena: &'ir Bump,
    pub registry: &'b mut Registry<RoughType<'ast>>,
    pub name_lookup: &'b mut PreFlattenNameLookup,
    pub import_bucket: &'b mut ImportBucket<'ir>,
    pub root: &'ast [Spanned<ast::Item<'ast>>],
}

impl<'ir, 'ast> RoughStage<'ir, '_, 'ast> {
    pub fn lower(mut self) {
        for item in self.root {
            self.lower_item(self.name_lookup.root, item);
        }
    }

    pub fn lower_item(&mut self, scope: ScopeId, item: &'ast ast::Item<'ast>) {
        match item {
            ast::Item::Use(u) => self.lower_use(scope, u),
            ast::Item::Module(m) => self.lower_mod(scope, m),
            ast::Item::Entity(e) => self.introduce_type(scope, e.name, RoughTypeItem::Entity(e)),
            ast::Item::Record(r) => self.introduce_type(scope, r.name, RoughTypeItem::Record(r)),
            ast::Item::Enum(e) => self.introduce_type(scope, e.name, RoughTypeItem::Enum(e)),
            ast::Item::Arch(_) => {}
        }
    }

    fn lower_use(&mut self, scope: ScopeId, u: &'ast ast::Use) {
        let Spanned(path, span) = &u.path;

        let import_scope = match path.1 {
            ast::PathStart::Root => self.name_lookup.root,
            ast::PathStart::Local => scope,
        };

        let id = self.import_bucket.insert(Import {
            scope: import_scope,
            strategy: LookupStrategy::Indirect,
            path: self
                .arena
                .alloc_slice_fill_iter(path.0.iter().map(|seg| seg.0)),
            span: *span,
            target_scope: scope,
        });
        self.name_lookup
            .introduce(scope, path.0.last().unwrap().0, Resolvable::Import(id));
    }

    fn lower_mod(&mut self, scope: ScopeId, m: &'ast ast::Module<'ast>) {
        let sub_scope = self.name_lookup.sub_scope(scope);

        let module = self.registry.modules.insert(Module {
            name: m.name,
            scope: sub_scope,
        });
        self.name_lookup.introduce(
            scope,
            m.name,
            Resolvable::Resolved(Resolved::Module(module)),
        );

        for i in &m.items {
            self.lower_item(sub_scope, i);
        }
    }

    fn introduce_type(&mut self, scope: ScopeId, name: Ident, t: RoughTypeItem<'ast>) {
        let id = self.registry.types.insert(RoughType(scope, t));

        self.name_lookup
            .introduce(scope, name, Resolvable::Resolved(Resolved::Type(id)));
    }
}
