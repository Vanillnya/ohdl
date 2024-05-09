use std::mem::MaybeUninit;

use crate::{
    ast::PathStart,
    ir::{
        import_bucket::{ImportBucket, ImportId},
        name_lookup::{
            LookupScope, PostFlattenNameLookup, PreFlattenNameLookup, Resolvable, Resolved,
        },
        registry::Registry,
    },
    message::Message,
    MESSAGES,
};

use super::rough::types::RoughType;

pub struct FlattenLookupStage<'ir, 'b, 'ast> {
    pub registry: &'b Registry<RoughType<'ast>>,
    pub name_lookup: PreFlattenNameLookup,
    pub import_bucket: ImportBucket<'ir>,
    pub resolvables: Vec<ImportId>,
}

impl<'ir> FlattenLookupStage<'ir, '_, '_> {
    pub fn lower(mut self) -> Option<PostFlattenNameLookup> {
        self.build_start_dependencies();

        while let Some(id) = self.resolvables.pop() {
            let import = &mut self.import_bucket.imports[id];
            'single_import: loop {
                let is_terminal = import.path.len() == 1;
                let segment = import.path[0];
                match self
                    .name_lookup
                    .lookup_ignore(
                        import.scope,
                        &segment,
                        import.start,
                        |r| matches!(r, Resolvable::Import(i) if *i == id),
                    )
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

        // TODO: is it really smart to convert all here or
        //       should we just let them as Resolvable and
        //       resolve them via the bucket on-demand?
        let mut successful_flattening = true;
        let scopes = self.name_lookup.scopes.map(|_, scope| LookupScope {
            entries: scope
                .entries
                .into_iter()
                .map(|(symbol, (span, resolvable))| {
                    (
                        symbol,
                        (
                            span,
                            match resolvable {
                                Resolvable::Import(i) => {
                                    let import = &self.import_bucket.imports[i];
                                    MESSAGES.report(Message::stuck_on_import(import.path[0]));
                                    successful_flattening = false;
                                    #[allow(invalid_value)]
                                    unsafe {
                                        // SAFETY: we just set `successful_flattening` to false, so we don't use this anymore.
                                        MaybeUninit::uninit().assume_init()
                                    }
                                }
                                Resolvable::Resolved(r) => r,
                            },
                        ),
                    )
                })
                .collect(),
            parent: scope.parent,
        });
        if successful_flattening {
            Some(PostFlattenNameLookup {
                scopes,
                root: self.name_lookup.root,
            })
        } else {
            None
        }
    }

    fn build_start_dependencies(&mut self) {
        for (id, import) in self.import_bucket.imports.iter() {
            match self.name_lookup.lookup_ignore(
                import.scope,
                &import.path[0],
                import.start,
                |r| matches!(r, Resolvable::Import(i) if *i == id),
            ) {
                Some(Resolvable::Import(dep)) => self.import_bucket.dependants[*dep].push(id),
                _ => self.resolvables.push(id),
            }
        }
    }
}
