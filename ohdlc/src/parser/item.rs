use crate::{
    ast::{
        item::{Entity, Item, ItemBase, Port, PortKind, Use},
        span::{Spanned, WithSpan},
    },
    lexer::TokenKind,
    spanned,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    pub fn parse_item(&mut self) -> PResult<Item<'s>> {
        let span = self.span_enter();

        let base = if self.eat_token(TokenKind::KwEntity)? {
            ItemBase::Entity(self.parse_entity()?)
        } else if self.eat_token(TokenKind::KwArch)? {
            todo!()
        } else if self.eat_token(TokenKind::KwUse)? {
            ItemBase::Use(self.parse_use()?)
        } else {
            panic!("unknown ding")
        };

        let span = self.span_leave(span);

        Ok(Item {
            base: base.with_span(span),
        })
    }

    pub fn parse_entity(&mut self) -> PResult<Entity<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::OpenCurly)?;

        let mut ports = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            let span_port = self.span_enter();

            let kind = if self.eat_token(TokenKind::KwIn)? {
                Spanned(PortKind::Input, self.prev_span())
            } else if self.eat_token(TokenKind::KwOut)? {
                Spanned(PortKind::Output, self.prev_span())
            } else {
                panic!("wtf")
            };

            let name = self.ident()?;
            self.consume(TokenKind::Colon)?;
            let r#type = spanned!(self { self.parse_type() })?;

            let span_port = self.span_leave(span_port);

            ports.push(Port { kind, name, r#type }.with_span(span_port));

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;
        Ok(Entity { name, ports })
    }

    pub fn parse_use(&mut self) -> PResult<Use<'s>> {
        let path = self.parse_path()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Use { path })
    }
}
