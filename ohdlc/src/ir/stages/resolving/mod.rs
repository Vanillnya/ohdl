use std::collections::VecDeque;

use crate::ir::{
    name_lookup::{PostImportNameLookup, PreImportNameLookup},
    name_resolution::{ImportId, NameResolution},
    registry::Registry,
};

pub struct ResolvingLowering<'ir, 'b> {
    pub registry: &'b Registry<'ir>,
    pub name_lookup: PreImportNameLookup,
    pub name_resolution: &'b mut NameResolution<'ir>,
    pub queue: VecDeque<ImportId>,
}

impl<'ir> ResolvingLowering<'ir, '_> {
    pub fn lower(mut self) -> PostImportNameLookup {
        todo!()
    }
}
