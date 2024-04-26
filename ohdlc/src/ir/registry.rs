use surotto::simple::SimpleSurotto;

use super::{
    modules::{Module, ModuleId},
    types::{Type, TypeId},
};

#[derive(Debug, Default)]
pub struct Registry<'ir> {
    pub modules: SimpleSurotto<ModuleId, Module>,
    pub types: SimpleSurotto<TypeId, Type<'ir>>,
}
