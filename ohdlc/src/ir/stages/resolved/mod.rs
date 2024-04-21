use bumpalo::Bump;

use crate::ir::IR;

pub struct ResolvedLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir>,
}

impl<'ir> ResolvedLowering<'ir, '_> {
    pub fn lower(self) {}
}
