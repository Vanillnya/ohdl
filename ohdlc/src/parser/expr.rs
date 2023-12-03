use crate::{
    ast::expr::{BinaryOperator, Expr},
    lexer::TokenKind,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    /// ### Parses an [`Expr`]
    pub fn parse_expr(&mut self) -> PResult<Expr<'s>> {
        self.parse_expr_(0)
    }

    fn parse_expr_(&mut self, precedence: usize) -> PResult<Expr<'s>> {
        let left = self.parse_expr_atom()?;

        let op = match self.kind()? {
            TokenKind::KwAnd => Some(BinaryOperator::And),
            TokenKind::KwOr => Some(BinaryOperator::Or),
            TokenKind::KwNand => Some(BinaryOperator::Nand),
            TokenKind::KwNor => Some(BinaryOperator::Nor),
            TokenKind::KwXor => Some(BinaryOperator::Xor),
            TokenKind::KwXnor => Some(BinaryOperator::Xnor),
            _ => None,
        };
        if let Some(op) = op {
            self.bump();
            let prec = precedence + 1; // when impl right-associative operators, for them it's just `precedence`;
            let right = self.parse_expr_(prec)?;
            Ok(Expr::Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator: op,
            })
        } else {
            Ok(left)
        }
    }

    fn parse_expr_atom(&mut self) -> PResult<Expr<'s>> {
        if self.eat_token(TokenKind::OpenParen)? {
            let expr = self.parse_expr()?;
            self.consume(TokenKind::CloseParen)?;
            Ok(expr)
        } else {
            Ok(Expr::Primary(self.ident()?))
        }
    }
}
