use bumpalo::Bump;

use crate::ir::IR;

pub struct ResolveLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> ResolveLowering<'ir, '_> {
    pub fn lower(mut self) {
        while let Some(import) = self.ir.imports.pop_front() {
            println!("{import:?}");
        }
    }
}
