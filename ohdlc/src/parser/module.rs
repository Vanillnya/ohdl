use crate::{ast::Module, spanned};

use super::{PResult, Parser};

impl<'s, 'a> Parser<'s, 'a> {
    /// ### Parses a [`Module`]
    pub fn parse_module(&mut self) -> PResult<Module<'a>> {
        let mut items = vec![];

        while self.has_next() {
            let item = spanned!(self { self.parse_item()? });
            items.push(item);
        }

        Ok(Module { items })
    }
}
