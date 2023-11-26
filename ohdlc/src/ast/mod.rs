use self::span::Spanned;

pub mod item;
pub mod span;
pub mod ty;

pub type Ident<'s> = Spanned<&'s str>;
