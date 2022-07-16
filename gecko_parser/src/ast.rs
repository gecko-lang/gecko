use crate::grammar::{GeckoParser, Rule};
use crate::expression::*;
use crate::statement::*;
use crate::node::*;
use crate::Token;

use pest_consume::Error;
use pest_consume::match_nodes;
use pest_consume::Parser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

fn token_from_node(input: Node) -> Result<Token> {
    let span: Span = Span::from_span(input.as_span());
    Ok(Token{ span, value: String::from(input.as_str().to_owned()) })
}

#[pest_consume::parser]
impl GeckoParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    // Tokens
    fn let_token(input: Node) -> Result<Token> { token_from_node(input) }
    fn func_token(input: Node) -> Result<Token> { token_from_node(input) }
    fn type_token(input: Node) -> Result<Token> { token_from_node(input) }
    fn return_token(input: Node) -> Result<Token> { token_from_node(input) }
    fn extern_token(input: Node) -> Result<Token> { token_from_node(input) }
    
    fn rarrow(input: Node) -> Result<Token> { token_from_node(input) }
    fn larrow(input: Node) -> Result<Token> { token_from_node(input) }

    fn lparen(input: Node) -> Result<Token> { token_from_node(input) }
    fn rparen(input: Node) -> Result<Token> { token_from_node(input) }
    fn lbrace(input: Node) -> Result<Token> { token_from_node(input) }
    fn rbrace(input: Node) -> Result<Token> { token_from_node(input) }

    fn colon(input: Node) -> Result<Token> { token_from_node(input) }
    fn comma(input: Node) -> Result<Token> { token_from_node(input) }
    //

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

        // TODO: Implement binary operators with precedence using lazy static
        // See resources for further details
        Ok(match_nodes!(nodes;
                [string(s)] => Box::new(s),
                [identifier(i)] => Box::new(i),
                [integer_decimal(i)] => Box::new(i)
        ))
    }

    fn block(input: Node) -> Result<Block> {
        let span: Span = Span::from_span(input.as_span());
        let mut lb: Option<Token> = None;
        let mut stmts: Vec<Box<dyn Statement>> = vec!();
        let mut rb: Option<Token> = None;

        for n in input.into_children().peekable() {
            match n.as_rule() {
                Rule::lbrace => lb = Some(GeckoParser::lbrace(n).unwrap()),
                Rule::rbrace => rb = Some(GeckoParser::rbrace(n).unwrap()),

                Rule::function_definition => stmts.push(Box::new(GeckoParser::function_definition(n).unwrap())),
                _ => {}
            }
        }

        assert_ne!(lb.is_none() || rb.is_none(), true);

        Ok(Block{
            lbrace: lb.unwrap(),
            stmts,
            rbrace: rb.unwrap(),
            span 
        })
    }
    
    fn parameter(input: Node) -> Result<Parameter> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [identifier(id), colon(c), identifier(ty_s)] => {
                let ty_span = ty_s.span;
                let type_specifier = TypeSpecifier{ id: ty_s, span: ty_span };
                Parameter{ id, colon_token: c, ty: type_specifier, span }
            }
        ))
    }

    fn parameter_list(input: Node) -> Result<ParameterList> {
        let span: Span = Span::from_span(input.as_span());

        let mut lp: Option<Token> = None;
        let mut params: Vec<(Parameter, Option<Token>)> = vec!();
        let mut rp: Option<Token> = None;

        let mut next_param: Option<Parameter> = None;

        for n in input.into_children().peekable() {
            match n.as_rule() {
                Rule::lparen => {
                    lp = Some(GeckoParser::lparen(n).unwrap());
                },
                Rule::parameter => {
                    next_param = Some(GeckoParser::parameter(n).unwrap());
                },
                Rule::comma => {
                    params.push((next_param.unwrap(), Some(GeckoParser::comma(n).unwrap())));
                    next_param = None;
                },
                Rule::rparen => {
                    if params.len() > 0 {
                        assert_eq!(next_param.is_none(), false);
                        params.push((next_param.unwrap(), None));
                        next_param = None;
                    }
                    
                    rp = Some(GeckoParser::rparen(n).unwrap());
                },
                _ => {}
            }
        }

        assert_ne!(lp.is_none() || rp.is_none(), true);

        Ok(ParameterList{
            lparen: lp.unwrap(),
            parameters: params,
            rparen: rp.unwrap(),
            span: span
        })
    }

    fn output(input: Node) -> Result<Output> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [rarrow(rarrow), identifier(id)] => {
                let ty_span = id.span;
                let ty: TypeSpecifier = TypeSpecifier{ id, span: ty_span };
                Output{
                    rarrow,
                    ty,
                    span
                }
            }
        ))
    }

    fn function_definition(input: Node) -> Result<impl Statement> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [func_token(func_token), identifier(id), parameter_list(params), output(output), block(block)] => {
                let sig_span = Span{ start: func_token.span.start, end: output.span.end };
                let sig = Signature{ func_token, id, params, output, span: sig_span };
                
                FunctionDefinition{
                    sig,
                    block,
                    span
                }
            }
        ))
    }

    // TODO: Import external modules for use in current file
    //
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
    //
    // TODO: Use statement when namespaces are implemented
    //       to move member of another namespace into
    //       the current namespace
    //
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

    fn file(input: Node) -> Result<File> {
        let span: Span = Span::from_span(input.as_span());
        let nodes = { input.into_children() };
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        for node in nodes {
            let rule = node.as_rule();
            match rule {
                Rule::EOI => {},
                Rule::function_definition => statements.push(Box::new(Self::function_definition(node)?)),
                // Rule::import_statement => statements.push(Box::new(Self::import_statement(node)?)),
                // Rule::use_statement => statements.push(Box::new(Self::use_statement(node)?)),
                _ => {}
            }
        }

        Ok(File{ stmts: statements, span: Some(span) })
    }
}

#[derive(Copy, Clone)]
//#[derive(Debug)]
pub struct LineColumn {
    line: usize,
    column: usize
}

#[derive(Copy, Clone)]
//#[derive(Debug)]
pub struct Span {
    start: LineColumn,
    end: LineColumn
}

impl Span {
    pub fn from_span(span: pest::Span) -> Self {
        let start = span.start_pos().line_col();
        let end = span.end_pos().line_col();

        Self{
            start: LineColumn{ line: start.0, column: start.1 },
            end: LineColumn{ line: end.0, column: end.1 }
        }
    }
}

pub fn parse_gecko(input_str: &str) -> Result<File> {
    let inputs = GeckoParser::parse_with_userdata(Rule::file, input_str, ())?;
    let input = inputs.single()?;

    GeckoParser::file(input)
}