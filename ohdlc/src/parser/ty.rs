use crate::{
    ast::ty::{Path, PathSegment, Type},
    lexer::TokenKind,
    spanned,
};

use super::{PResult, Parser};

impl<'s> Parser<'s> {
    pub fn parse_path(&mut self) -> PResult<Path<'s>> {
        let mut segments = vec![];
        segments.push(PathSegment(self.ident()?));

        while self.kind()? == TokenKind::ColonColon {
            self.bump();
            segments.push(PathSegment(self.ident()?));
        }

        Ok(Path(segments))
    }

    pub fn parse_type(&mut self) -> PResult<Type<'s>> {
        let path = spanned!(self { self.parse_path() })?;
        Ok(Type { path })
    }
}
