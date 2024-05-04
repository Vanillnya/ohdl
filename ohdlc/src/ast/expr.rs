use crate::symbol::Ident;

#[derive(Debug)]
pub enum Expr<'ast> {
    Binary {
        left: &'ast Expr<'ast>,
        right: &'ast Expr<'ast>,
        operator: BinaryOperator,
    },
    Unary {
        operator: UnaryOperator,
        value: &'ast Expr<'ast>,
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
