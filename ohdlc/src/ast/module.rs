use super::Item;
use crate::span::Spanned;

pub struct Module<'a> {
    pub items: Vec<Spanned<Item<'a>>>,
}
