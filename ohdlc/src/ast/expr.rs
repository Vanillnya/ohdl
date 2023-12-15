use crate::symbol::Ident;

#[derive(Debug)]
pub enum Expr<'a> {
    Binary {
        left: &'a Expr<'a>,
        right: &'a Expr<'a>,
        operator: BinaryOperator,
    },
    Unary {
        operator: UnaryOperator,
        value: &'a Expr<'a>,
    },
    Primary(Ident),
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
