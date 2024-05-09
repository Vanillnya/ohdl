#![deny(rust_2018_idioms)]

use message::Messages;

// TODO: can we get unused errors back? :3
pub mod ast;
pub mod ir;
pub mod lexer;
pub mod message;
pub mod parser;
pub mod span;
pub mod symbol;

pub static MESSAGES: Messages = Messages::new();

#[derive(Clone)]
pub struct Source<'s>(pub String, pub &'s str);
