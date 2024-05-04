use surotto::{simple::SimpleSurotto, simple_key};

use super::modules::{Module, ModuleId};

#[derive(Debug)]
pub struct Registry<T> {
    pub modules: SimpleSurotto<ModuleId, Module>,
    pub types: SimpleSurotto<TypeId, T>,
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            modules: SimpleSurotto::new(),
            types: SimpleSurotto::new(),
        }
    }
}

simple_key!(
    pub struct TypeId;
);
