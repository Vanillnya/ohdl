use bumpalo::Bump;

use crate::{
    ir::{
        name_resolution::{ImportResult, PathStart},
        resolving::{Resolvable, Resolved, ScopeId},
        IR,
    },
    message::Message,
    symbol::Ident,
    MESSAGES,
};

pub struct ResolveLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> ResolveLowering<'ir, '_> {
    pub fn lower(mut self) {
        while let Some(id) = self.ir.name_resolution.queue.pop_front() {
            let import = &self.ir.name_resolution.imports[id];
            let import = match import {
                ImportResult::InProgress(i) => i,
                ImportResult::Finished(_) => {
                    unreachable!("finished imports shouldn't scheduled")
                }
            };
            let segment = import.path.first().unwrap();
            println!("{segment:?}");

            let Some(resolvable) =
                self.ir
                    .resolving_scopes
                    .find_resolvable(import.scope, &segment, import.start)
            else {
                MESSAGES.report(Message::could_not_resolve(*segment));
                continue;
            };

            let resolved = match *resolvable {
                Resolvable::Import(i) => {
                    match &self.ir.name_resolution.imports[i] {
                        ImportResult::InProgress(_) => {
                            // TODO: check if we haven't done any progress since last time to prevent circular infinite load
                            self.ir.name_resolution.queue.push_back(id);
                            None
                        }
                        ImportResult::Finished(r) => Some(*r),
                    }
                }
                Resolvable::Resolved(r) => Some(r),
            };

            if let Some(r) = resolved {
                match r {
                    Resolved::Type(t) => {
                        // TODO: check if it is not the last thing
                        self.ir.name_resolution.imports[id] =
                            ImportResult::Finished(Resolved::Type(t));
                    }
                    Resolved::Module(m) => {
                        let module = &self.ir.modules[m];
                        let sub_path = &import.path[1..];
                        if sub_path.is_empty() {
                            self.ir.name_resolution.imports[id] =
                                ImportResult::Finished(Resolved::Module(m));
                        } else {
                            let import = match &mut self.ir.name_resolution.imports[id] {
                                ImportResult::InProgress(i) => i,
                                ImportResult::Finished(_) => {
                                    unreachable!("we already checked that earlier")
                                }
                            };
                            import.scope = module.scope;
                            import.start = PathStart::Direct;
                            import.path = sub_path;
                            self.ir.name_resolution.queue.push_back(id);
                        }
                    }
                }
            }
        }
    }
}
