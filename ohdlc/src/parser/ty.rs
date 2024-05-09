use crate::{
    ast::{Path, PathSegment, PathStart, Type},
    lexer::TokenKind,
    spanned,
};

use super::{PResult, Parser};

impl<'s, 'a> Parser<'s, 'a> {
    /// ### Parses a [`Path`]
    pub fn parse_path(&mut self) -> PResult<Path> {
        let mut segments = vec![];

        let path_start = if self.kind()? == TokenKind::ColonColon {
            self.bump();
            PathStart::Root
        } else {
            PathStart::Local
        };

        segments.push(PathSegment(self.ident()?));

        while self.kind()? == TokenKind::ColonColon {
            self.bump();
            segments.push(PathSegment(self.ident()?));
        }

        Ok(Path(segments, path_start))
    }

    /// ### Parses a [`Type`]
    pub fn parse_type(&mut self) -> PResult<Type> {
        let path = spanned!(self { self.parse_path()? });
        Ok(Type { path })
    }
}
