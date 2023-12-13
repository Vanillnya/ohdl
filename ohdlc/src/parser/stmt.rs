use crate::{
    ast::{
        AssignStmt, Connector, PlaceLink, PlaceLinkInternal, PlaceStmt, PortKind, Spanned, Stmt,
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
        let (kind, arrow_span) = match self.next()? {
            Spanned(TokenKind::LeftBigArrow, s) => (PortKind::Input, s),
            Spanned(TokenKind::RightBigArrow, s) => (PortKind::Output, s),
            token => {
                self.messages
                    .report(Message::unexpected_token(token.1, "'<=' or '=>'", token.0))?
            }
        };

        let internal = match kind {
            PortKind::Input => PlaceLinkInternal::Ingoing(spanned!(self { self.parse_expr()? })),
            PortKind::Output => PlaceLinkInternal::Outgoing(spanned!(self {
                if self.eat_token(TokenKind::KwSignal)? {
                    Connector::NewSignal(self.ident()?)
                } else {
                    Connector::Ref(self.ident()?)
                }
            })),
        };

        Ok(PlaceLink {
            src,
            arrow_span,
            link: internal,
        })
    }

    /// ### Parses an [`AssignStmt`]
    pub fn parse_stmt_assign(&mut self) -> PResult<AssignStmt<'s>> {
        let assignee = self.ident()?;
        self.consume(TokenKind::LeftBigArrow)?;
        let expr = self.parse_expr()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(AssignStmt {
            assignee,
            value: expr,
        })
    }
}
