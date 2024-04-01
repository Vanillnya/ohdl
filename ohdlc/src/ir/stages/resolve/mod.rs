use bumpalo::Bump;

use crate::{
    ir::{resolving::Resolvable, IR},
    message::Message,
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

            let resolvable = {
                let mut scope = &self.ir.resolving_scopes[import.src];
                'resolve: loop {
                    match scope.entries.get(&segment) {
                        Some(resolvable) => break 'resolve Some(resolvable),
                        None => match scope.parent {
                            Some(p) => scope = &self.ir.resolving_scopes[p],
                            None => break 'resolve None,
                        },
                    }
                }
            };
            let Some(resolvable) = resolvable else {
                MESSAGES.report(Message::could_not_resolve(*segment));
                continue;
            };

            match *resolvable {
                Resolvable::Type(t) => todo!(),
                Resolvable::Module(m) => {
                    let module = &self.ir.modules[m];
                    import.src = module.scope;
                    import.path = &import.path[1..];
                }
                Resolvable::Import(i) => todo!(),
            }
        }
    }
}
