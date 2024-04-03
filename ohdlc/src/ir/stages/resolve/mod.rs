use bumpalo::Bump;

use crate::{
    ir::{
        name_resolution::PathStart,
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
                Resolvable::Type(t) => todo!(),
                Resolvable::Module(m) => {
                    let module = &self.ir.modules[m];
                    import.scope = module.scope;
                    import.start = PathStart::Direct;
                    import.path = &import.path[1..];
                }
                Resolvable::Import(i) => todo!(),
            }
        }
    }
}
