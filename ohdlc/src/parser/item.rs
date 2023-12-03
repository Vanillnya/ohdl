use crate::{
    ast::{
        item::{Arch, Entity, Enum, Field, Item, ItemBase, Port, PortKind, Record, Use},
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
        let base = spanned!(self { self.parse_item_base()? });

        Ok(Item { base })
    }

    /// ### Parses an [`ItemBase`]
    fn parse_item_base(&mut self) -> PResult<ItemBase<'s>> {
        match self.next()? {
            Spanned(TokenKind::KwUse, _) => self.parse_use().map(ItemBase::Use),
            Spanned(TokenKind::KwEntity, _) => self.parse_entity().map(ItemBase::Entity),
            Spanned(TokenKind::KwArch, _) => self.parse_arch().map(ItemBase::Arch),
            Spanned(TokenKind::KwRecord, _) => self.parse_record().map(ItemBase::Record),
            Spanned(TokenKind::KwEnum, _) => self.parse_enum().map(ItemBase::Enum),
            token => self.messages.report(Message::unexpected_token(
                token.1,
                "'entity' or 'arch' or 'use'",
                token.0,
            ))?,
        }
    }

    /// ### Parses an [`Use`]
    ///
    /// Assumes that the `use` keyword was already consumed.
    pub fn parse_use(&mut self) -> PResult<Use<'s>> {
        let path = self.parse_path()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Use { path })
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
            let ty = spanned!(self { self.parse_type()? });

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
        let ty = spanned!(self { self.parse_type()? });

        self.consume(TokenKind::OpenCurly)?;

        let mut stmts = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            stmts.push(spanned!(self { self.parse_stmt()? }));
        }

        self.consume(TokenKind::CloseCurly)?;

        Ok(Arch { name, ty, stmts })
    }

    /// ### Parses a [`Record`]
    ///
    /// Assumes that the `record` keyword was already consumed.
    pub fn parse_record(&mut self) -> PResult<Record<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::OpenCurly)?;

        let mut fields = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            let span_field = self.span_enter();

            let name = self.ident()?;
            self.consume(TokenKind::Colon)?;
            let ty = spanned!(self { self.parse_type()? });

            let span_field = self.span_leave(span_field);

            fields.push(Field { name, ty }.with_span(span_field));

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;
        Ok(Record { name, fields })
    }

    /// ### Parses an [`Enum`]
    ///
    /// Assumes that the `enum` keyword was already consumed.
    pub fn parse_enum(&mut self) -> PResult<Enum<'s>> {
        let name = self.ident()?;
        self.consume(TokenKind::OpenCurly)?;

        let mut variants = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            let variant = self.ident()?;

            variants.push(variant);

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;
        Ok(Enum { name, variants })
    }
}
