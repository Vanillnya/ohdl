use std::cell::RefCell;

use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        modules::Module,
        name_resolution::{Import, ImportId, ImportResult},
        resolving::{Resolvable, Resolved, ScopeId},
        stage::IRStage,
        types::{Entity, Enum, Field, Port, Record, Type, TypeId, Variant},
        IR,
    },
    span::Spanned,
};

pub struct UnresolvedStage;

impl IRStage for UnresolvedStage {
    type ResolvingEntry = Resolvable;
}

impl UnresolvedStage {
    pub fn lower<'ir>(arena: &'ir Bump, ast: &[Spanned<ast::Item<'_>>]) -> IR<'ir, Self> {
        let mut ir = IR::<'ir, Self>::new();
        let mut lowering = UnresolvedLowering { arena, ir: &mut ir };
        lowering.lower(ast);
        ir
    }
}

pub struct UnresolvedLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir, UnresolvedStage>,
}

impl<'ir> UnresolvedLowering<'ir, '_> {
    pub fn lower(mut self, root: &[Spanned<ast::Item<'_>>]) {
        for item in root {
            self.lower_item(self.ir.resolving_scopes.root, item);
        }
    }

    pub fn lower_item(&mut self, scope: ScopeId, item: &ast::Item<'_>) {
        match item {
            ast::Item::Use(u) => self.lower_use(scope, u),
            ast::Item::Module(m) => self.lower_mod(scope, m),
            ast::Item::Entity(e) => self.lower_entity(scope, e),
            ast::Item::Record(r) => self.lower_record(scope, r),
            ast::Item::Enum(e) => self.introduce_type(scope, |type_id| {
                Type::Enum(Enum {
                    type_id,
                    name: e.name,
                    variants: self
                        .arena
                        .alloc_slice_fill_iter(e.variants.iter().map(|&ident| Variant { ident })),
                })
            }),
            ast::Item::Arch(_) => {}
        }
    }

    fn lower_use(&mut self, scope: ScopeId, u: &ast::Use) {
        let id = self.schedule_resolution_of_path(scope, &u.path);
        self.ir
            .introduce(scope, u.path.0 .0.last().unwrap().0, Resolvable::Import(id));
    }

    fn lower_mod(&mut self, scope: ScopeId, m: &ast::Module<'_>) {
        let sub_scope = self.ir.resolving_scopes.sub_scope(scope);

        let module = self.ir.modules.insert(Module {
            name: m.name,
            scope: sub_scope,
        });
        self.ir.introduce(
            scope,
            m.name,
            Resolvable::Resolved(Resolved::Module(module)),
        );

        for i in &m.items {
            self.lower_item(sub_scope, i);
        }
    }

    fn lower_entity(&mut self, scope: ScopeId, entity: &ast::Entity) {
        let ports = entity
            .ports
            .iter()
            .map(|port| Port {
                kind: port.kind.0,
                name: port.name,
                ty: self.schedule_resolution_of_path(scope, &port.ty.path),
            })
            .collect();

        self.introduce_type(scope, |type_id| {
            Type::Entity(Entity {
                type_id,
                name: entity.name,
                ports,
            })
        })
    }

    fn lower_record(&mut self, scope: ScopeId, record: &ast::Record) {
        let fields = record
            .fields
            .iter()
            .map(|field| Field {
                name: field.name,
                ty: self.schedule_resolution_of_path(scope, &field.ty.path),
            })
            .collect();

        self.introduce_type(scope, |type_id| {
            Type::Record(Record {
                type_id,
                name: record.name,
                fields,
            })
        })
    }

    fn introduce_type<F>(&mut self, scope: ScopeId, f: F)
    where
        F: FnOnce(TypeId) -> Type<'ir>,
    {
        let id = self.ir.types.insert_with(f);

        let name = self.ir.types[id].name();
        self.ir
            .introduce(scope, name, Resolvable::Resolved(Resolved::Type(id)));
    }

    fn schedule_resolution_of_path(
        &mut self,
        scope: ScopeId,
        path: &Spanned<ast::Path>,
    ) -> ImportId {
        let Spanned(path, span) = path;
        let import = Import {
            scope,
            start: path.1,
            path: self
                .arena
                .alloc_slice_fill_iter(path.0.iter().map(|seg| seg.0)),
            progress: true,
            span: *span,
        };
        let id = self
            .ir
            .name_resolution
            .imports
            .insert(RefCell::new(ImportResult::InProgress(import)));
        self.ir.name_resolution.queue.push_back(id);
        id
    }
}
