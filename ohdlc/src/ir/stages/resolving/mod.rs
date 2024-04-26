use std::collections::VecDeque;

use crate::{
    ast::PathStart,
    ir::{
        modules::Modules,
        name_resolution::{ImportId, ImportResult, NameResolution},
        resolving::{Resolvable, Resolved, ResolvingScopes},
    },
    message::Message,
    MESSAGES,
};

pub struct ResolvingLowering<'ir, 'b> {
    pub modules: &'b Modules,
    pub resolving_scopes: &'b ResolvingScopes,
    pub name_resolution: &'b mut NameResolution<'ir>,
    pub queue: VecDeque<ImportId>,
}

impl<'ir> ResolvingLowering<'ir, '_> {
    pub fn lower(mut self) {
        while let Some(id) = self.queue.pop_front() {
            let import_res = &mut *self.name_resolution.imports[id].borrow_mut();
            let import = match import_res {
                ImportResult::InProgress(i) => i,
                ImportResult::Finished(_) => {
                    unreachable!("finished imports shouldn't be scheduled")
                }
            };
            let segment = import.path.first().unwrap();
            println!("{segment:?}");

            import.progress = false;

            let Some(resolvable) =
                self.resolving_scopes
                    .find_resolvable(import.scope, &segment, import.start, id)
            else {
                MESSAGES.report(Message::could_not_resolve(*segment));
                continue;
            };

            let resolved = match *resolvable {
                Resolvable::Import(i) => {
                    if i == id {
                        panic!("Hek {segment:?} {resolvable:?}");
                    }
                    match &mut *self.name_resolution.imports[i].borrow_mut() {
                        ImportResult::InProgress(ipi) => {
                            if ipi.progress {
                                import.progress = true;
                                self.queue.push_back(id);
                                None
                            } else {
                                MESSAGES.report(Message::stuck_on_import(*segment, ipi.span));
                                None
                            }
                        }
                        ImportResult::Finished(r) => Some(*r),
                    }
                }
                Resolvable::Resolved(r) => Some(r),
            };

            // TODO: do we need an else branch here?
            if let Some(r) = resolved {
                match r {
                    Resolved::Type(t) => {
                        // TODO: check if it is not the last thing
                        *import_res = ImportResult::Finished(Resolved::Type(t));
                    }
                    Resolved::Module(m) => {
                        let module = &self.modules[m];
                        let sub_path = &import.path[1..];
                        if sub_path.is_empty() {
                            *import_res = ImportResult::Finished(Resolved::Module(m));
                        } else {
                            import.scope = module.scope;
                            import.start = PathStart::Direct;
                            import.path = sub_path;
                            self.queue.push_back(id);
                        }
                    }
                }
            }
        }
    }
}
