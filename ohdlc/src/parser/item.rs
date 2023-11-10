use crate::{
    ast::{
        item::{Entity, Item, ItemKind, Port, PortKind},
        span::{Spanned, WithSpan},
    },
    lexer::TokenKind,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    pub fn parse_item(&mut self) -> PResult<Spanned<Item<'s>>> {
        let span = self.span_begin();

        let kind = if self.eat_token(TokenKind::KwEntity)? {
            ItemKind::Entity(self.parse_entity()?)
        } else if self.eat_token(TokenKind::KwArch)? {
            todo!()
        } else {
            panic!("unknown ding")
        };

        let span = self.span_end(span);

        Ok(Item { kind }.with_span(span))
    }

    pub fn parse_entity(&mut self) -> PResult<Entity<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::OpenCurly)?;

        let mut ports = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            let span = self.span_begin();

            let kind = if self.eat_token(TokenKind::KwIn)? {
                Spanned(PortKind::Input, self.prev_span())
            } else if self.eat_token(TokenKind::KwOut)? {
                Spanned(PortKind::Output, self.prev_span())
            } else {
                panic!("wtf")
            };

            let name = self.ident()?;
            self.consume(TokenKind::Colon)?;
            let r#type = self.ident()?;

            let span = self.span_end(span);

            ports.push(Port { kind, name, r#type }.with_span(span));

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;
        Ok(Entity { name, ports })
    }
}
