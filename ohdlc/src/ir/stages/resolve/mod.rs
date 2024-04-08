use bumpalo::Bump;

use crate::{
    ast::PathStart,
    ir::{
        name_resolution::ImportResult,
        resolving::{Resolvable, Resolved},
        IR,
    },
    message::Message,
    MESSAGES,
};

pub struct ResolveLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> ResolveLowering<'ir, '_> {
    pub fn lower(self) {
        while let Some(id) = self.ir.name_resolution.queue.pop_front() {
            let import_res = &mut *self.ir.name_resolution.imports[id].borrow_mut();
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
                self.ir
                    .resolving_scopes
                    .find_resolvable(import.scope, &segment, import.start)
            else {
                MESSAGES.report(Message::could_not_resolve(*segment));
                continue;
            };

            let resolved = match *resolvable {
                Resolvable::Import(i) => {
                    // TODO: when replacing with UnsafeCell, make sure that i != id
                    match &mut *self.ir.name_resolution.imports[i].borrow_mut() {
                        ImportResult::InProgress(ipi) => {
                            if ipi.progress {
                                import.progress = true;
                                self.ir.name_resolution.queue.push_back(id);
                                None
                            } else {
                                None
                            }
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
                        *import_res = ImportResult::Finished(Resolved::Type(t));
                    }
                    Resolved::Module(m) => {
                        let module = &self.ir.modules[m];
                        let sub_path = &import.path[1..];
                        if sub_path.is_empty() {
                            *import_res = ImportResult::Finished(Resolved::Module(m));
                        } else {
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
