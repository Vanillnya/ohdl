use bumpalo::Bump;

use crate::{ast, ir::{name_lookup::PostFlattenNameLookup, registry::{Registry, TypeId}}};

use self::types::{Entity, Port, RefinedType};

use super::rough::types::RoughType;

pub mod types;

pub struct RefineTypesStage<'ir, 'b> {
    pub arena: &'ir Bump,
    pub name_lookup: &'b PostFlattenNameLookup,
}

impl<'ir, 'ast> RefineTypesStage<'ir, '_> {
    pub fn lower(self, registry: Registry<RoughType<'ast>>) -> Registry<RefinedType<'ir>> {
        Registry { modules: registry.modules, types: registry.types.map(|id, rough| self.refine_type(id, rough)) }
    }

    fn refine_type(&self, id: TypeId, rough: RoughType<'ast>) -> RefinedType<'ir> {
        match rough {
            RoughType::Entity(e) => self.refine_entity(id, e),
            _ => todo!(),
        }
    }
    
    fn refine_entity(&self, id: TypeId, e: &ast::Entity) -> RefinedType<'ir> {
        let ports = e.ports.iter().map(|port| Port {
            kind: port.kind.0,
            name: port.name,
            ty: ()
        }).collect();

        RefinedType::Entity(Entity { type_id: id, name: e.name, ports })
    }
}