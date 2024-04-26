use bumpalo::Bump;

use crate::ir::{resolving::Resolved, stage::IRStage, IR};

pub struct ResolvedStage;

impl IRStage for ResolvedStage {
    type ResolvingEntry = Resolved;
}

pub struct ResolvedLowering<'ir, 'b> {
    pub arena: &'ir Bump,
    pub ir: &'b mut IR<'ir, ResolvedStage>,
}

impl<'ir> ResolvedLowering<'ir, '_> {
    pub fn lower(self) {}
}
