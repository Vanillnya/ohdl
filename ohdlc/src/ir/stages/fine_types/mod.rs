use bumpalo::Bump;

use crate::ir::IR;

pub struct FineTypesLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> FineTypesLowering<'ir, '_> {
    pub fn lower(self) {}
}
