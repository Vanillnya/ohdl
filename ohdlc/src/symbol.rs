use lasso::{Spur, ThreadedRodeo};
use once_cell::sync::Lazy;
use std::fmt::Debug;

use crate::span::Spanned;

static SYMBOL_POOL: Lazy<ThreadedRodeo> = Lazy::new(ThreadedRodeo::default);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Symbol(Spur);
impl Symbol {
    pub fn intern(val: impl AsRef<str>) -> Self {
        Self(SYMBOL_POOL.get_or_intern(val))
    }

    pub fn get(&self) -> &str {
        SYMBOL_POOL.resolve(&self.0)
    }
}
impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.get())
    }
}

pub type Ident = Spanned<Symbol>;
