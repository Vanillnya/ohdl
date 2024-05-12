use bumpalo::Bump;

use crate::{
    ast,
    ir::{
        import_bucket::LookupStrategy,
        name_lookup::{PostFlattenNameLookup, Resolved, ScopeId},
        registries::{ArchId, EntityId, ModuleRegistry, RoughArchRegistry},
    },
    message::Message,
    MESSAGES,
};

use super::rough::registries::RoughArch;

pub struct ArchitectureStage<'ir, 'b, 'ast> {
    pub arena: &'ir Bump,
    pub name_lookup: &'b PostFlattenNameLookup,
    pub module_reg: &'b ModuleRegistry,
    pub arch_reg: &'b RoughArchRegistry<'ast>,
}

impl<'ir, 'ast> ArchitectureStage<'ir, '_, 'ast> {
    pub fn lower(self) {
        for (id, arch) in self.arch_reg.iter() {
            self.lower_arch(id, arch)
        }
    }

    fn lower_arch(&self, id: ArchId, arch: &RoughArch<'ast>) {
        let ty = self.lookup_entity(arch.0, &arch.1.ty);
        println!("Implementing arch '{}' for {:?}", arch.1.name.0.get(), ty);
    }

    // TODO: similar to one in `architectures`, fix duplicate pls
    fn lookup_entity(&self, scope: ScopeId, ty: &ast::Type) -> Option<EntityId> {
        let mut lookup_scope = match ty.path.0 .1 {
            ast::PathStart::Root => self.name_lookup.root,
            ast::PathStart::Local => scope,
        };

        let mut path = ty.path.0 .0.iter().peekable();
        let mut is_start = true;
        while let Some(segment) = path.next() {
            let is_terminal = path.peek().is_none();
            let segment = segment.0;

            let lookup = self.name_lookup.lookup(
                lookup_scope,
                &segment,
                if is_start {
                    LookupStrategy::Indirect
                } else {
                    LookupStrategy::Direct
                },
            );
            match (is_terminal, lookup) {
                (false, Some(Resolved::Entity(_))) => {
                    MESSAGES.report(Message::use_continues_after_type(segment.1));
                    return None;
                }
                (false, Some(Resolved::Module(m))) => {
                    lookup_scope = self.module_reg[*m].scope;
                }

                (true, Some(Resolved::Entity(t))) => return Some(*t),
                (true, Some(Resolved::Module(_))) => {
                    MESSAGES.report(Message::wrong_path_end(segment, "Entity", "Module"));
                    return None;
                }

                (_, Some(Resolved::Type(_))) => {
                    // TODO: better error, this can also appear at non-end position
                    MESSAGES.report(Message::wrong_path_end(segment, "Entity", "Type"))
                }
                (_, None) => {
                    MESSAGES.report(Message::could_not_resolve(segment));
                    return None;
                }
            }

            is_start = false;
        }

        return None;
    }
}
