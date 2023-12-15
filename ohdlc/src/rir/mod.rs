mod item;
use std::fmt::Debug;

pub mod lowering;

pub use item::*;
use lasso::{Spur, ThreadedRodeo};
use once_cell::sync::Lazy;

static IDENT_POOL: Lazy<ThreadedRodeo> = Lazy::new(ThreadedRodeo::default);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Ident(Spur);
impl Ident {
    pub fn intern(val: impl AsRef<str>) -> Self {
        Self(IDENT_POOL.get_or_intern(val))
    }

    pub fn get(&self) -> &str {
        IDENT_POOL.resolve(&self.0)
    }
}
impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.get())
    }
}
