use bumpalo::Bump;

use crate::ir::{name_lookup::PostFlattenNameLookup, registry::Registry};

use self::types::RefinedType;

use super::rough::types::RoughType;

pub mod types;

pub struct RefineTypesStage<'ir, 'b, 'ast> {
    pub arena: &'ir Bump,
    pub registry: Registry<RoughType<'ast>>,
    pub name_lookup: &'b PostFlattenNameLookup,
}

impl<'ir, 'ast> RefineTypesStage<'ir, '_, 'ast> {
    pub fn lower(mut self) -> Registry<RefinedType<'ir>> {
        todo!()
    }
}