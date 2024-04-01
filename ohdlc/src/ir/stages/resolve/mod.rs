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
            println!("{import:?}");
        }
    }
}
