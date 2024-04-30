use crate::{
    ast::PathStart,
    ir::{
        import_bucket::{ImportBucket, ImportId},
        name_lookup::{PostFlattenNameLookup, PreFlattenNameLookup, Resolvable, Resolved},
        registry::Registry,
    },
    message::Message,
    MESSAGES,
};

pub struct FlattenLookupStage<'ir, 'b> {
    pub registry: &'b Registry<'ir>,
    pub name_lookup: PreFlattenNameLookup,
    pub import_bucket: ImportBucket<'ir>,
    pub resolvables: Vec<ImportId>,
}

impl<'ir> FlattenLookupStage<'ir, '_> {
    pub fn lower(mut self) -> PostFlattenNameLookup {
        self.build_start_dependencies();

        while let Some(id) = self.resolvables.pop() {
            let import = &mut self.import_bucket.imports[id];
            'single_import: loop {
                let is_terminal = import.path.len() == 1;
                let segment = import.path[0];
                match self
                    .name_lookup
                    .lookup(import.scope, &segment, import.start)
                    .cloned()
                {
                    Some(Resolvable::Import(_)) => {
                        unreachable!("`resolvables` only contains ready-to-resolve imports")
                    }
                    Some(Resolvable::Resolved(r)) => {
                        if is_terminal {
                            self.name_lookup[import.target_scope]
                                .entries
                                .insert(segment.0, (segment.1, Resolvable::Resolved(r)));
                            for dependant in self.import_bucket.dependants[id].iter() {
                                self.resolvables.push(*dependant);
                            }
                        } else {
                            match r {
                                Resolved::Type(_) => {
                                    MESSAGES.report(Message::use_continues_after_type(segment.1));
                                }
                                Resolved::Module(m) => {
                                    let module = &self.registry.modules[m];
                                    let sub_path = &import.path[1..];
                                    import.scope = module.scope;
                                    import.start = PathStart::Direct;
                                    import.path = sub_path;

                                    continue 'single_import;
                                }
                            }
                        }
                    }
                    None => {
                        MESSAGES.report(Message::could_not_resolve(segment));
                    }
                }
                break 'single_import;
            }
        }

        for cyclic in self.resolvables {
            let import = &self.import_bucket.imports[cyclic];
            MESSAGES.report(Message::stuck_on_import(import.path[0]));
        }
        todo!()
    }

    fn build_start_dependencies(&mut self) {
        for (id, import) in self.import_bucket.imports.iter() {
            match self
                .name_lookup
                .lookup(import.scope, &import.path[0], import.start)
            {
                Some(Resolvable::Import(dep)) => self.import_bucket.dependants[*dep].push(id),
                _ => self.resolvables.push(id),
            }
        }
    }
}
