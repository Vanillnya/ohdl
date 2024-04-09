use crate::{
    ast::{AssignStmt, Connector, PlaceLink, PlaceLinkInternal, PlaceStmt, PortKind, Stmt},
    lexer::TokenKind,
    message::Message,
    span::Spanned,
    spanned,
};

use super::{PResult, Parser};

macro_rules! branch {
    ($self:ident $($variant:path => { $($b:tt)* }),+,) => {
        $(
            {
                let state = $self.state();
                if let Ok(branch) = { $($b)* } {
                    return Ok($variant(branch));
                } else {
                    $self.recover_state(state);
                }
            }
        )+
        {
            // TODO: better errors
            println!("shit");
            Err(vec![])
        }
    };
}

impl<'s, 'a> Parser<'s, 'a> {
    /// ### Parses an [`Stmt`]
    pub fn parse_stmt(&mut self) -> PResult<Stmt<'a>> {
        branch! { self
            Stmt::Place => { self.parse_stmt_place() },
            Stmt::Assign => { self.parse_stmt_assign() },
        }
    }

    /// ### Parses an [`PlaceStmt`]
    pub fn parse_stmt_place(&mut self) -> PResult<PlaceStmt<'a>> {
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
    fn parse_place_link(&mut self) -> PResult<PlaceLink<'a>> {
        let src = self.ident()?;
        let (kind, arrow_span) = match self.next()? {
            Spanned(TokenKind::LeftBigArrow, s) => (PortKind::Input, s),
            Spanned(TokenKind::RightBigArrow, s) => (PortKind::Output, s),
            token => {
                return Err(vec![Message::unexpected_token(
                    token.1,
                    "'<=' or '=>'",
                    token.0,
                )]);
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
    pub fn parse_stmt_assign(&mut self) -> PResult<AssignStmt<'a>> {
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
