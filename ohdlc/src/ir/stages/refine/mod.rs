use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        import_bucket::LookupStrategy,
        name_lookup::{PostFlattenNameLookup, Resolved, ScopeId},
        registries::{
            EntityId, ModuleRegistry, RefinedEntityRegistry, RefinedTypeRegistry,
            RoughEntityRegistry, RoughTypeRegistry, TypeId,
        },
    },
    message::Message,
    MESSAGES,
};

use self::registries::{Entity, Enum, Field, Port, Record, Type, Variant};

use super::rough::registries::RoughType;

pub mod registries;

pub struct RefineStage<'ir, 'b> {
    pub arena: &'ir Bump,
    pub name_lookup: &'b PostFlattenNameLookup,
    pub module_registry: &'b ModuleRegistry,
}

impl<'ir, 'ast> RefineStage<'ir, '_> {
    pub fn refine_types(&self, registry: RoughTypeRegistry<'ast>) -> RefinedTypeRegistry<'ir> {
        registry.map(|id, rough| self.refine_type(id, rough))
    }

    pub fn refine_entities(
        &self,
        registry: RoughEntityRegistry<'ast>,
    ) -> RefinedEntityRegistry<'ir> {
        registry.map(|id, rough| self.refine_entity(id, rough.0, rough.1))
    }

    fn refine_type(&self, id: TypeId, rough: RoughType<'ast>) -> Type<'ir> {
        match rough {
            RoughType::Record(scope, r) => self.refine_record(id, scope, r),
            RoughType::Enum(e) => self.refine_enum(id, e),
        }
    }

    fn refine_entity(&self, id: EntityId, scope: ScopeId, e: &ast::Entity) -> Entity<'ir> {
        let ports = e.ports.iter().map(|port| Port {
            kind: port.kind.0,
            name: port.name,
            ty: self.lookup_type(scope, &port.ty),
        });

        Entity {
            type_id: id,
            name: e.name,
            ports: self.arena.alloc_slice_fill_iter(ports),
        }
    }

    fn refine_record(&self, id: TypeId, scope: ScopeId, r: &ast::Record) -> Type<'ir> {
        let fields = r.fields.iter().map(|field| Field {
            name: field.name,
            ty: self.lookup_type(scope, &field.ty),
        });

        Type::Record(Record {
            type_id: id,
            name: r.name,
            fields: self.arena.alloc_slice_fill_iter(fields),
        })
    }

    fn refine_enum(&self, id: TypeId, e: &ast::Enum) -> Type<'ir> {
        let varaints = e.variants.iter().map(|&ident| Variant { ident });

        Type::Enum(Enum {
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

                (_, Some(Resolved::Entity(_))) => {
                    // TODO: better error, this can also appear at non-end position
                    MESSAGES.report(Message::wrong_path_end(segment, "Type", "Entity"))
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
