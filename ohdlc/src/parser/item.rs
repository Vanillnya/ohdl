use std::marker::PhantomData;

use crate::{
    ast::{
        item::{Entity, Item, ItemKind},
        span::{Spanned, WithSpan},
    },
    TokenValue,
};

use super::{Parser, TokenRef};

impl<'s> Parser<'s> {
    pub fn parse_item(&mut self) -> Result<Spanned<Item<'s>>, ()> {
        let span = self.span_begin()?;
        match self.current()? {
            Some(TokenRef(TokenValue::KwEntity, _)) => Ok(Item {
                kind: ItemKind::Entity(self.parse_entity()?),
            }
            .with_span(self.span_end(span)?)),
            Some(TokenRef(TokenValue::KwArch, _)) => panic!("arch"),
            Some(_) => panic!("unexpected begin of decl"),
            None => panic!("end"),
        }
    }

    pub fn parse_entity(&mut self) -> Result<Entity<'s>, ()> {
        Ok(Entity {
            _phantom: PhantomData,
        })
    }
}
