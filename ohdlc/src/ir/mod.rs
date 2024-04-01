use surotto::simple_key;

mod modules;
mod name_resolution;
mod resolving;
mod types;

pub mod stages;

simple_key!(
    pub struct ScopeId;
);
