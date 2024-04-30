use crate::ir::{
    import_bucket::ImportBucket,
    name_lookup::{PostFlattenNameLookup, PreFlattenNameLookup},
    registry::Registry,
};

pub struct FlattenLookupStage<'ir, 'b> {
    pub registry: &'b Registry<'ir>,
    pub name_lookup: PreFlattenNameLookup,
    pub import_bucket: ImportBucket<'ir>,
}

impl<'ir> FlattenLookupStage<'ir, '_> {
    pub fn lower(mut self) -> PostFlattenNameLookup {
        todo!()
    }
}
