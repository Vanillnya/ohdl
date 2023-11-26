use crate::{
    ast::{
        item::{Arch, Entity, Item, ItemBase, Port, PortKind, Use},
        span::{Spanned, WithSpan},
    },
    lexer::TokenKind,
    message::Message,
    spanned,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    /// ### Parses an [`Item`]
    pub fn parse_item(&mut self) -> PResult<Item<'s>> {
        let span = self.span_enter();

        let base = if self.eat_token(TokenKind::KwEntity)? {
            ItemBase::Entity(self.parse_entity()?)
        } else if self.eat_token(TokenKind::KwArch)? {
            ItemBase::Arch(self.parse_arch()?)
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

    /// ### Parses an [`Entity`]
    ///
    /// Assumes that the `entity` keyword was already consumed.
    pub fn parse_entity(&mut self) -> PResult<Entity<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::OpenCurly)?;

        let mut ports = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            let span_port = self.span_enter();

            let kind = match self.next()? {
                Spanned(TokenKind::KwIn, s) => PortKind::Input.with_span(s),
                Spanned(TokenKind::KwOut, s) => PortKind::Output.with_span(s),
                token => self.messages.report(Message::unexpected_token(
                    token.1,
                    "'in' or 'out'",
                    token.0,
                ))?,
            };

            let name = self.ident()?;
            self.consume(TokenKind::Colon)?;
            let ty = spanned!(self { self.parse_type() })?;

            let span_port = self.span_leave(span_port);

            ports.push(Port { kind, name, ty }.with_span(span_port));

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;
        Ok(Entity { name, ports })
    }

    /// ### Parses an [`Arch`]
    ///
    /// Assumes that the `arch` keyword was already consumed.
    pub fn parse_arch(&mut self) -> PResult<Arch<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::KwFor)?;
        let ty = spanned!(self { self.parse_type() })?;

        self.consume(TokenKind::OpenCurly)?;

        let mut stmts = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            stmts.push(spanned!(self { self.parse_stmt() })?);
        }

        self.consume(TokenKind::CloseCurly)?;

        Ok(Arch { name, ty, stmts })
    }

    /// ### Parses an [`Use`]
    ///
    /// Assumes that the `use` keyword was already consumed.
    pub fn parse_use(&mut self) -> PResult<Use<'s>> {
        let path = self.parse_path()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Use { path })
    }
}
