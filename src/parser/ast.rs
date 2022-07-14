use crate::grammar::{GeckoParser, Rule};
use crate::expression::*;

use crate::statement::*;

use pest_consume::Error;
use pest_consume::match_nodes;
use pest_consume::Parser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl GeckoParser {

}

pub struct LineColumn {
    line: usize,
    column: usize
}

pub struct Span {
    start: LineColumn,
    end: LineColumn
}

impl Span {
    pub fn from_span(span: pest::Span) -> Self {
        let start = span.start_pos().line_col();
        let end = span.end_pos().line_col();

        Self { start: LineColumn{ line: start.0, column: start.1 }, end: LineColumn{ line: end.0, column: end.1 } }
    }
}

pub fn parse_gecko(input_str: &str) -> Result<Vec<Box<dyn Statement>>> {
    let inputs = ElapseParser::parse_with_userdata(Rule::file, input_str, ())?;
    let input = inputs.single()?;
    GeckoParser::file(input)
}