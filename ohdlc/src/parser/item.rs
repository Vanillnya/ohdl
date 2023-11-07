use std::marker::PhantomData;

use crate::{
    ast::{
        item::{Entity, Item, ItemKind},
        span::{Spanned, WithSpan},
    },
    TokenValue,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    pub fn parse_item(&mut self) -> PResult<Spanned<Item<'s>>> {
        let span = self.span_begin()?;
        match self.current()? {
            Some(Spanned(TokenValue::KwEntity, _)) => Ok(Item {
                kind: ItemKind::Entity(self.parse_entity()?),
            }
            .with_span(self.span_end(span)?)),
            Some(Spanned(TokenValue::KwArch, _)) => panic!("arch"),
            Some(_) => panic!("unexpected begin of decl"),
            None => panic!("end"),
        }
    }

    pub fn parse_entity(&mut self) -> PResult<Entity<'s>> {
        Ok(Entity {
            _phantom: PhantomData,
        })
    }
}
