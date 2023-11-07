use std::marker::PhantomData;

#[derive(Debug)]
pub struct Item<'s> {
    pub kind: ItemKind<'s>,
}

#[derive(Debug)]
pub enum ItemKind<'s> {
    Entity(Entity<'s>),
    Arch(Arch<'s>),
}

#[derive(Debug)]
pub struct Entity<'s> {
    pub _phantom: PhantomData<&'s ()>,
}

#[derive(Debug)]
pub struct Arch<'s> {
    pub _phantom: PhantomData<&'s ()>,
}
