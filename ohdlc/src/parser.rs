use std::ops::Range;

use ariadne::{Label, Report, ReportKind};
use logos::SpannedIter;

use crate::{Source, TokenValue};

pub mod decl;

type TokenIter<'source> = itertools::MultiPeek<SpannedIter<'source, TokenValue>>;

pub struct Parser<'s> {
    source: Source<'s>,
    tokens: TokenIter<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: Source<'s>, tokens: SpannedIter<'s, TokenValue>) -> Self {
        Self {
            source,
            tokens: itertools::multipeek(tokens),
        }
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Result<Option<(&TokenValue, &Range<usize>)>, ()> {
        match self.tokens.peek() {
            Some((Ok(value), span)) => Ok(Some((value, span))),
            Some((Err(_), span)) => {
                print_report(
                    ReportKind::Error,
                    &self.source,
                    "Unknown Token",
                    "Whatever this is here",
                    span.clone(),
                );
                Err(())
            }
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> Result<Option<(TokenValue, Range<usize>)>, ()> {
        match self.tokens.next() {
            Some((Ok(value), span)) => Ok(Some((value, span))),
            Some((Err(_), span)) => {
                print_report(
                    ReportKind::Error,
                    &self.source,
                    "Unknown Token",
                    "Whatever this is here",
                    span,
                );
                Err(())
            }
            None => Ok(None),
        }
    }
}

pub trait Parselet: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ()>;
}

fn print_report<S1: Into<String>, S2: Into<String>>(
    report_kind: ReportKind,
    source: &Source,
    message: S1,
    label_message: S2,
    location: Range<usize>,
) {
    let filename = source.0.as_str();

    let report = Report::build(report_kind, filename, location.start)
        .with_message(message.into())
        .with_label(Label::new((filename, location)).with_message(label_message.into()))
        .finish();

    report
        .eprint((filename, ariadne::Source::from(source.1)))
        .unwrap();
}
