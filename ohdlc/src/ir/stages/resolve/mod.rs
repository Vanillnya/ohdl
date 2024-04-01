use bumpalo::Bump;

use crate::ir::IR;

pub struct ResolveLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> ResolveLowering<'ir, '_> {
    pub fn lower(mut self) {
        while let Some(id) = self.ir.name_resolution.queue.pop_front() {
            let import = &self.ir.name_resolution.imports[id];
            let segment = import.path.first().unwrap();
            if let Some(thing) = self.ir.resolving_scopes[import.src].entries.get(&segment) {
                println!("{thing:?}");
                println!("{segment:?}");
            }
        }
    }
}
