use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        import_bucket::LookupStrategy,
        name_lookup::{PostFlattenNameLookup, Resolved, ScopeId},
        registry::{ModuleRegistry, TypeId, TypeRegistry},
    },
    message::Message,
    MESSAGES,
};

use self::types::{Entity, Enum, Field, Port, Record, RefinedType, Variant};

use super::rough::types::{RoughType, RoughTypeItem};

pub mod types;

pub struct RefineTypesStage<'ir, 'b> {
    pub arena: &'ir Bump,
    pub name_lookup: &'b PostFlattenNameLookup,
    pub module_registry: &'b ModuleRegistry,
}

impl<'ir, 'ast> RefineTypesStage<'ir, '_> {
    pub fn lower(self, registry: TypeRegistry<RoughType<'ast>>) -> TypeRegistry<RefinedType<'ir>> {
        registry.map(|id, rough| self.refine_type(id, rough))
    }

    fn refine_type(&self, id: TypeId, rough: RoughType<'ast>) -> RefinedType<'ir> {
        match rough.1 {
            RoughTypeItem::Entity(e) => self.refine_entity(id, rough.0, e),
            RoughTypeItem::Record(r) => self.refine_record(id, rough.0, r),
            RoughTypeItem::Enum(e) => self.refine_enum(id, e),
        }
    }

    fn refine_entity(&self, id: TypeId, scope: ScopeId, e: &ast::Entity) -> RefinedType<'ir> {
        let ports = e.ports.iter().map(|port| Port {
            kind: port.kind.0,
            name: port.name,
            ty: self.lookup_type(scope, &port.ty),
        });

        RefinedType::Entity(Entity {
            type_id: id,
            name: e.name,
            ports: self.arena.alloc_slice_fill_iter(ports),
        })
    }

    fn refine_record(&self, id: TypeId, scope: ScopeId, r: &ast::Record) -> RefinedType<'ir> {
        let fields = r.fields.iter().map(|field| Field {
            name: field.name,
            ty: self.lookup_type(scope, &field.ty),
        });

        RefinedType::Record(Record {
            type_id: id,
            name: r.name,
            fields: self.arena.alloc_slice_fill_iter(fields),
        })
    }

    fn refine_enum(&self, id: TypeId, e: &ast::Enum) -> RefinedType<'ir> {
        let varaints = e.variants.iter().map(|&ident| Variant { ident });

        RefinedType::Enum(Enum {
            type_id: id,
            name: e.name,
            variants: self.arena.alloc_slice_fill_iter(varaints),
        })
    }

    fn lookup_type(&self, scope: ScopeId, ty: &ast::Type) -> Option<TypeId> {
        let mut lookup_scope = match ty.path.0 .1 {
            ast::PathStart::Root => self.name_lookup.root,
            ast::PathStart::Local => scope,
        };

        let mut path = ty.path.0 .0.iter().peekable();
        let mut is_start = true;
        while let Some(segment) = path.next() {
            let is_terminal = path.peek().is_none();
            let segment = segment.0;

            let lookup = self.name_lookup.lookup(
                lookup_scope,
                &segment,
                if is_start {
                    LookupStrategy::Indirect
                } else {
                    LookupStrategy::Direct
                },
            );
            match (is_terminal, lookup) {
                (false, Some(Resolved::Type(_))) => {
                    MESSAGES.report(Message::use_continues_after_type(segment.1));
                    return None;
                }
                (false, Some(Resolved::Module(m))) => {
                    lookup_scope = self.module_registry[*m].scope;
                }

                (true, Some(Resolved::Type(t))) => return Some(*t),
                (true, Some(Resolved::Module(_))) => {
                    MESSAGES.report(Message::wrong_path_end(segment, "Type", "Module"));
                    return None;
                }

                (_, None) => {
                    MESSAGES.report(Message::could_not_resolve(segment));
                    return None;
                }
            }

            is_start = false;
        }

        return None;
    }
}
