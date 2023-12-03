use crate::{
    ast::expr::{BinaryOperator, Expr, UnaryOperator},
    lexer::TokenKind,
};

use super::{PResult, Parser};

macro_rules! derivation {
    (fn $name:ident match ($in:ty) for $out:ty {
        $($from:path => $to:path),*$(,)?
    }) => {
        #[inline(always)]
        fn $name(input: $in) -> Option<$out> {
            #[allow(dead_code)]
            fn _assert_outputs(x: $out) {
                match x {
                    $($to => ()),*,
                };
            }
            match input {
                $($from => Some($to)),*,
                _ => None,
            }
        }
    };
}

impl<'s> Parser<'s> {
    /// ### Parses an [`Expr`]
    pub fn parse_expr(&mut self) -> PResult<Expr<'s>> {
        self.parse_expr_(0)
    }

    fn parse_expr_(&mut self, precedence: usize) -> PResult<Expr<'s>> {
        derivation!(fn bin_op match (TokenKind) for BinaryOperator {
            TokenKind::KwAnd => BinaryOperator::And,
            TokenKind::KwOr => BinaryOperator::Or,
            TokenKind::KwNand => BinaryOperator::Nand,
            TokenKind::KwNor => BinaryOperator::Nor,
            TokenKind::KwXor => BinaryOperator::Xor,
            TokenKind::KwXnor => BinaryOperator::Xnor,
        });

        let left = self.parse_expr_unary()?;

        if let Some(op) = bin_op(self.kind()?) {
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

    fn parse_expr_unary(&mut self) -> PResult<Expr<'s>> {
        derivation!(fn unary_op match (TokenKind) for UnaryOperator {
            TokenKind::KwNot => UnaryOperator::Not,
        });

        if let Some(op) = unary_op(self.kind()?) {
            self.bump();
            let atom = self.parse_expr_atom()?;
            Ok(Expr::Unary {
                operator: op,
                value: Box::new(atom),
            })
        } else {
            self.parse_expr_atom()
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
