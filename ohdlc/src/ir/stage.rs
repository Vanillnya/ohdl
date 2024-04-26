use std::fmt::Debug;

use bumpalo::Bump;

use super::IR;

pub trait IRStage: Sized {
    type ResolvingEntry: Debug;
}
