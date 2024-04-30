use std::collections::VecDeque;

use crate::ir::{
    name_lookup::{PostFlattenNameLookup, PreFlattenNameLookup},
    name_resolution::{ImportId, NameResolution},
    registry::Registry,
};

pub struct FlattenLookupStage<'ir, 'b> {
    pub registry: &'b Registry<'ir>,
    pub name_lookup: PreFlattenNameLookup,
    pub name_resolution: &'b mut NameResolution<'ir>,
    pub queue: VecDeque<ImportId>,
}

impl<'ir> FlattenLookupStage<'ir, '_> {
    pub fn lower(mut self) -> PostFlattenNameLookup {
        todo!()
    }
}
