use super::Ident;

#[derive(Debug)]
pub enum Expr<'s> {
    Binary {
        left: Box<Expr<'s>>,
        right: Box<Expr<'s>>,
        operator: BinaryOperator,
    },
    Primary(Ident<'s>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    And,
    Xor,
    Or,
}
