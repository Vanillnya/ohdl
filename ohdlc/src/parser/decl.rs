use ariadne::ReportKind;

use crate::{ast_span::Span, print_report, TokenValue};

use super::{Parselet, Parser, TokenRef};

pub enum Decl<'s> {
    Entity(EntityDecl<'s>),
    Arch(ArchDecl),
}

impl<'s> Parselet<'s> for Decl<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ()> {
        match parser.current()? {
            Some(TokenRef(TokenValue::KwEntity, _)) => Ok(Decl::Entity(EntityDecl::parse(parser)?)),
            Some(TokenRef(TokenValue::KwArch, _)) => panic!("arch"),
            Some(_) => panic!("unexpected begin of decl"),
            None => panic!("end"),
        }
    }
}

pub struct EntityDecl<'s>(Span, NameParselet<'s>);

impl<'s> Parselet<'s> for EntityDecl<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ()> {
        let span = Span::start(parser)?;
        parser.next(); // KwEntity
        let name = NameParselet::parse(parser)?;
        // TODO:
        let span = Span::with_start(parser, span)?;
        Ok(EntityDecl(span, name))
    }
}

pub struct ArchDecl();

pub struct NameParselet<'s>(Span, &'s str);

impl<'s> Parselet<'s> for NameParselet<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ()> {
        match parser.current()? {
            Some(TokenRef(TokenValue::Ident, span)) => {
                // SAFETY: it exists
                let _ = parser.next();
                Ok(NameParselet(span, parser.slice(span)))
            }
            Some(TokenRef(tv, span)) => {
                let error = format!("Expected {:?}, but got {tv:?}", TokenValue::Ident);
                print_report(
                    &parser.source,
                    ReportKind::Error,
                    span,
                    error,
                    format!("Expected {:?} here", TokenValue::Ident),
                );
                Err(())
            }
            None => {
                let end = parser.source.1.len();
                print_report(
                    &parser.source,
                    ReportKind::Error,
                    Span::from(end..end),
                    format!("Expected {:?}, but source ended early", TokenValue::Ident),
                    format!("Expected {:?} here", TokenValue::Ident),
                );
                Err(())
            }
        }
    }
}
