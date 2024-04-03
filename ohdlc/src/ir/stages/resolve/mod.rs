use bumpalo::Bump;

use crate::{
    ir::{
        name_resolution::{ImportResult, PathStart},
        resolving::{Resolvable, ScopeId},
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
            let import = &mut self.ir.name_resolution.imports[id];
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

            match *resolvable {
                Resolvable::Type(t) => {}
                Resolvable::Module(m) => {
                    let module = &self.ir.modules[m];
                    let sub_path = &import.path[1..];
                    if sub_path.is_empty() {
                        println!("Finished!");
                    } else {
                        import.scope = module.scope;
                        import.start = PathStart::Direct;
                        import.path = sub_path;
                        self.ir.name_resolution.queue.push_back(id);
                    }
                }
                Resolvable::Import(i) => {}
            }
        }
    }
}
