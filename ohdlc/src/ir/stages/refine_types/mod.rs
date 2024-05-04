use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        name_lookup::PostFlattenNameLookup,
        registry::{Registry, TypeId},
    },
};

use self::types::{Entity, Enum, Field, Port, Record, RefinedType, Variant};

use super::rough::types::RoughType;

pub mod types;

pub struct RefineTypesStage<'ir, 'b> {
    pub arena: &'ir Bump,
    pub name_lookup: &'b PostFlattenNameLookup,
}

impl<'ir, 'ast> RefineTypesStage<'ir, '_> {
    pub fn lower(self, registry: Registry<RoughType<'ast>>) -> Registry<RefinedType<'ir>> {
        Registry {
            modules: registry.modules,
            types: registry.types.map(|id, rough| self.refine_type(id, rough)),
        }
    }

    fn refine_type(&self, id: TypeId, rough: RoughType<'ast>) -> RefinedType<'ir> {
        match rough {
            RoughType::Entity(e) => self.refine_entity(id, e),
            RoughType::Record(r) => self.refine_record(id, r),
            RoughType::Enum(e) => self.refine_enum(id, e),
        }
    }

    fn refine_entity(&self, id: TypeId, e: &ast::Entity) -> RefinedType<'ir> {
        let ports = e.ports.iter().map(|port| Port {
            kind: port.kind.0,
            name: port.name,
            ty: (),
        });

        RefinedType::Entity(Entity {
            type_id: id,
            name: e.name,
            ports: self.arena.alloc_slice_fill_iter(ports),
        })
    }

    fn refine_record(&self, id: TypeId, r: &ast::Record) -> RefinedType<'ir> {
        let fields = r.fields.iter().map(|field| Field {
            name: field.name,
            ty: (),
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
}
