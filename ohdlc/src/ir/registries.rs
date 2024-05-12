use surotto::{simple::SimpleSurotto, simple_key};

use super::{
    modules::{Module, ModuleId},
    stages::{
        refine::registries::{Entity, Type},
        rough::registries::{RoughArch, RoughEntity, RoughType},
    },
};

pub type ModuleRegistry = SimpleSurotto<ModuleId, Module>;
pub type RoughTypeRegistry<'ast> = SimpleSurotto<TypeId, RoughType<'ast>>;
pub type RoughEntityRegistry<'ast> = SimpleSurotto<EntityId, RoughEntity<'ast>>;
pub type RoughArchRegistry<'ast> = SimpleSurotto<ArchId, RoughArch<'ast>>;
pub type RefinedTypeRegistry<'ir> = SimpleSurotto<TypeId, Type<'ir>>;
pub type RefinedEntityRegistry<'ir> = SimpleSurotto<EntityId, Entity<'ir>>;

simple_key!(
    pub struct TypeId;
);

simple_key!(
    pub struct EntityId;
);

simple_key!(
    pub struct ArchId;
);
