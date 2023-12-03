use super::Ident;

#[derive(Debug)]
pub enum Expr<'s> {
    Binary {
        left: Box<Expr<'s>>,
        right: Box<Expr<'s>>,
        operator: BinaryOperator,
    },
    Unary {
        operator: UnaryOperator,
        value: Box<Expr<'s>>,
    },
    Primary(Ident<'s>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Xnor,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
}
