use crate::{
    ast::expr::{BinaryOperator, Expr},
    lexer::TokenKind,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    /// ### Parses an [`Expr`]
    pub fn parse_expr(&mut self) -> PResult<Expr<'s>> {
        let primary = self.parse_expr_primary()?;
        self.parse_expr_(primary)
    }

    fn parse_expr_(&mut self, left: Expr<'s>) -> PResult<Expr<'s>> {
        let op = match self.kind()? {
            TokenKind::KwAnd => Some(BinaryOperator::And),
            TokenKind::KwXor => Some(BinaryOperator::Xor),
            TokenKind::KwOr => Some(BinaryOperator::Or),
            _ => None,
        };
        if let Some(op) = op {
            self.bump();
            let right = self.parse_expr_primary()?;
            Ok(Expr::Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator: op,
            })
        } else {
            Ok(left)
        }
    }

    fn parse_expr_primary(&mut self) -> PResult<Expr<'s>> {
        Ok(Expr::Primary(self.ident()?))
    }
}
