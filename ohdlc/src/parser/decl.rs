use crate::TokenValue;

use super::{Parselet, Parser};

pub enum Decl {
    Entity(EntityDecl),
    Arch(ArchDecl),
}

impl Parselet for Decl {
    fn parse(parser: &mut Parser) -> Result<Self, ()> {
        match parser.peek()? {
            Some((TokenValue::KwEntity, _)) => panic!("entity"),
            Some((TokenValue::KwArch, _)) => panic!("arch"),
            Some((_, _)) => panic!("unexpected begin of decl"),
            None => panic!("end"),
        }
    }
}

pub struct EntityDecl();
pub struct ArchDecl();
