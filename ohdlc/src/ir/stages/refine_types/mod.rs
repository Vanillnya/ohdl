use bumpalo::Bump;

use crate::{
    ast,
    ir::{
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
            RoughTypeItem::Enum(e) => self.refine_enum(id, rough.0, e),
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

    fn refine_enum(&self, id: TypeId, scope: ScopeId, e: &ast::Enum) -> RefinedType<'ir> {
        let varaints = e.variants.iter().map(|&ident| Variant { ident });

        RefinedType::Enum(Enum {
            type_id: id,
            name: e.name,
            variants: self.arena.alloc_slice_fill_iter(varaints),
        })
    }

    fn lookup_type(&self, mut scope: ScopeId, ty: &ast::Type) -> Option<TypeId> {
        for segment in ty.path.0 .0.iter() {
            let lookup = self.name_lookup.lookup(scope, &segment.0, ty.path.0 .1);
            match lookup {
                // TODO: check if path is not at end
                Some(Resolved::Type(t)) => return Some(*t),
                Some(Resolved::Module(m)) => {
                    scope = self.module_registry[*m].scope;
                }
                None => {
                    MESSAGES.report(Message::could_not_resolve(segment.0));
                    return None;
                }
            }
        }
        // TODO: report error when last one is not a type
        return None;
    }
}
