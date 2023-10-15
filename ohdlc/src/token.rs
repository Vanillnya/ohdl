#[derive(Debug)]
pub struct Token<'s>(pub RawToken<'s>);

#[derive(Debug)]
pub enum RawToken<'s> {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(&'s str),
}

#[derive(Debug)]
pub enum Keyword {
    Entity,
    In,
    Out,
}

#[derive(Debug)]
pub enum Symbol {
    OpenCurly,
    CloseCurly,
    Colon,
    Comma,
}
