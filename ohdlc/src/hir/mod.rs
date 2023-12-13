mod item;
use std::cell::RefCell;

pub use item::*;
use lasso::{Rodeo, Spur};
use once_cell::unsync::Lazy;

thread_local! {
    static IDENT_POOL: Lazy<RefCell<Rodeo>> = Lazy::new(|| {
        RefCell::new(Rodeo::default())
    });
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Ident(Spur);
impl Ident {
    pub fn intern(val: impl AsRef<str>) -> Self {
        let spur = IDENT_POOL.with(|pool| pool.borrow_mut().get_or_intern(val));
        Self(spur)
    }
}
