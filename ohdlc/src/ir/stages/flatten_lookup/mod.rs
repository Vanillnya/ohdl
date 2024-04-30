use crate::ir::{
    import_bucket::ImportBucket,
    name_lookup::{PostFlattenNameLookup, PreFlattenNameLookup, Resolvable},
    registry::Registry,
};

pub struct FlattenLookupStage<'ir, 'b> {
    pub registry: &'b Registry<'ir>,
    pub name_lookup: PreFlattenNameLookup,
    pub import_bucket: ImportBucket<'ir>,
}

impl<'ir> FlattenLookupStage<'ir, '_> {
    pub fn lower(mut self) -> PostFlattenNameLookup {
        self.build_start_dependencies();
        todo!()
    }

    fn build_start_dependencies(&mut self) {
        for (id, import) in self.import_bucket.imports.iter() {
            match self
                .name_lookup
                .lookup(import.scope, &import.path[0], import.start)
            {
                Some(Resolvable::Import(dep)) => self.import_bucket.dependants[*dep].push(id),
                _ => {}
            }
        }
    }
}
