use surotto::simple_key;

mod modules;
mod resolving;
mod types;

pub mod stages;

simple_key!(
    pub struct ScopeId;
);
