mod expr;
pub use expr::*;
mod item;
pub use item::*;
mod stmt;
pub use stmt::*;
mod ty;
pub use ty::*;

use crate::span::Spanned;

pub type Ident<'s> = Spanned<&'s str>;
