use crate::{
    ast::{
        item::PortKind,
        span::{Spanned, WithSpan},
        stmt::{AssignStmt, LinkDest, PlaceLink, PlaceStmt, Stmt},
    },
    lexer::TokenKind,
    message::Message,
    spanned,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    /// ### Parses an [`Stmt`]
    pub fn parse_stmt(&mut self) -> PResult<Stmt<'s>> {
        if self.eat_token(TokenKind::KwPlace)? {
            Ok(Stmt::Place(self.parse_stmt_place()?))
        } else {
            Ok(Stmt::Assign(self.parse_stmt_assign()?))
        }
    }

    /// ### Parses an [`PlaceStmt`]
    ///
    /// Assumes that the `place` keyword was already consumed.
    pub fn parse_stmt_place(&mut self) -> PResult<PlaceStmt<'s>> {
        let entity_ty = spanned!(self { self.parse_type()? });
        self.consume(TokenKind::OpenParen)?;
        let arch_ty = spanned!(self { self.parse_type()? });
        self.consume(TokenKind::CloseParen)?;

        self.consume(TokenKind::OpenCurly)?;

        let mut links = vec![];

        while self.kind()? != TokenKind::CloseCurly {
            links.push(spanned!(self { self.parse_place_link()? }));

            if !self.eat_token(TokenKind::Comma)? {
                break;
            }
        }

        self.consume(TokenKind::CloseCurly)?;

        Ok(PlaceStmt {
            entity_ty,
            arch_ty,
            links,
        })
    }

    /// ### Parses an [`PlaceLink`]
    fn parse_place_link(&mut self) -> PResult<PlaceLink<'s>> {
        let src = self.ident()?;
        let kind = match self.next()? {
            Spanned(TokenKind::LeftBigArrow, s) => PortKind::Input.with_span(s),
            Spanned(TokenKind::RightBigArrow, s) => PortKind::Output.with_span(s),
            token => {
                self.messages
                    .report(Message::unexpected_token(token.1, "'<=' or '=>'", token.0))?
            }
        };

        let dst = if self.eat_token(TokenKind::KwWire)? {
            LinkDest::NewWire(self.ident()?)
        } else {
            LinkDest::Ref(self.ident()?)
        };

        Ok(PlaceLink { src, kind, dst })
    }

    /// ### Parses an [`AssignStmt`]
    pub fn parse_stmt_assign(&mut self) -> PResult<AssignStmt<'s>> {
        todo!()
    }
}
