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
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    fn identifier(input: Node) -> Result<Identifier> {
        let span: Span = Span::from_span(input.as_span());
        Ok(Identifier{ span, name: input.as_str().to_owned() })
    }
    fn integer_decimal(input: Node) -> Result<Integer> {
        let span: Span = Span::from_span(input.as_span());
        Ok(Integer { span,
                     value: input.as_str().to_owned() })
    }
    fn string(input: Node) -> Result<Str> {
        let span: Span = Span::from_span(input.as_span());
        Ok(Str{ span, string: String::from(input.as_str().to_owned()) })
    }
    fn expression(input: Node) -> Result<Box<dyn Expression>> {
        let nodes = { input.into_children() };
        //if nodes.count() == 1 {
            Ok(match_nodes!(nodes;
                [string(s)] => Box::new(s),
                [identifier(i)] => Box::new(i),
                [integer_decimal(i)] => Box::new(i)
            ))
        //} 
    }

    fn function_definition(input: Node) -> Result<impl Statement> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [identifier(id), typed_identifer_list(params), return_type(return_type), block(block)] => {
                FunctionDefinition {
                    span,
                    id,
                    param_list,
                    return_type,
                    block
                }
            }
        ))
    }

    // fn import_statement(input: Node) -> Result<impl Statement> {
    //     let span: Span = Span::from_span(input.as_span());
    //     Ok(match_nodes!(input.into_children();
    //         [string(path)] => {
    //             import::Import {
    //                 span,
    //                 path
    //             }
    //         }
    //     ))
    // }
    // fn use_statement(input: Node) -> Result<impl Statement> {
    //     let span: Span = Span::from_span(input.as_span());
    //     Ok(match_nodes!(input.into_children();
    //         [scope_resolution(scope)] => {
    //             using::Use {
    //                 span,
    //                 scope
    //             }
    //         }
    //     ))
    // }
    fn file(input: Node) -> Result<Vec<Box<dyn Statement>>> {
        let nodes = { input.into_children() };
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        for node in nodes {
            let rule = node.as_rule();
            match rule {
                Rule::EOI => println!("end of input"),
                Rule::function_definition => statements.push(Box::new(Self::function_definition(node)?)),
                // Rule::import_statement => statements.push(Box::new(Self::import_statement(node)?)),
                // Rule::use_statement => statements.push(Box::new(Self::use_statement(node)?)),
                _ => {}
            }
        }

        for statement in &statements {
            println!("{}", statement.display());
        }
        Ok(statements)
    }
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
    let inputs = GeckoParser::parse_with_userdata(Rule::file, input_str, ())?;
    let input = inputs.single()?;
    GeckoParser::file(input)
}