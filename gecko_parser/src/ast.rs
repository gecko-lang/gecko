
use crate::grammar::{GeckoParser, Rule};
use crate::expression::*;
use crate::statement::*;
use crate::node::*;
use crate::Token;

use crate::colored::*;

use crate::precedence::*;
use pest_consume::{Error, match_nodes, Parser};

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

// Helper function for all grammar.pest rules that return a Token
fn token_from_node(input: Node) -> Result<Token> {
    let span: Span = Span::from_span(input.as_span());
    Ok(Token{ span, value: String::from(input.as_str().to_owned()) })
}

// Gecko Parser derived from PEST
#[pest_consume::parser]
impl GeckoParser {
    // End of input
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    // Rules that return Tokens
    fn let_token(input: Node) -> Result<Token> { token_from_node(input) }
    fn proc_token(input: Node) -> Result<Token> { token_from_node(input) }
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

    fn singlequote(input: Node) -> Result<Token> { token_from_node(input) }
    fn doublequote(input: Node) -> Result<Token> { token_from_node(input) }

    fn equals(input: Node) -> Result<Token> { token_from_node(input) }
    //

    fn identifier(input: Node) -> Result<Identifier> {
        let span: Span = Span::from_span(input.as_span());
        Ok(Identifier{ span, name: input.as_str().to_owned() })
    }

    fn boolean(input: Node) -> Result<Boolean> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [boolean_true(bool_true)] => Boolean { value: bool_true, span },
            [boolean_false(bool_false)] => Boolean { value: bool_false, span }
        ))
    }
    fn boolean_true(input: Node) -> Result<bool> {
        Ok(true)
    }
    fn boolean_false(input: Node) -> Result<bool> {
        Ok(false)
    }
    
    fn integer_zero(input: Node) -> Result<i128> {
        Ok(0)
    }

    // Decimal identifier. Expands to i128 to handle all possible integers
    fn integer_decimal(input: Node) -> Result<i128> {
        Ok( match input.as_str().to_owned().parse::<i128>() {
            Ok(int) => int,
            Err(error) => panic!("{:?}", error)
        })
    }

    // Integer which can be of any form but is stored as a decimal i128
    fn integer(input: Node) -> Result<Integer> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [integer_decimal(decimal)] => Integer{ value: decimal, span },
            [integer_zero(zero)] => Integer{ value: zero, span }
        ))
    }

    fn float(input: Node) -> Result<Float> {
        let span: Span = Span::from_span(input.as_span());
        Ok( match input.as_str().to_owned().parse::<f64>() {
            Ok(float) => Float { value: float, span },
            Err(error) => panic!{"{:?}", error}
        })
    }

    fn character(input: Node) -> Result<Character> {
        let span: Span = Span::from_span(input.as_span());
        
        Ok(match_nodes!(input.into_children();
            [singlequote(lquote), character, singlequote(rquote)] => {
                match character.as_str().to_owned().parse::<char>() {
                    Ok(content) => Character{ lquote, value: content as u8, rquote, span },
                    Err(error) => panic!{"{:?}", error}
                }
            }
        ))
    }

    // String value
    fn string(input: Node) -> Result<Str> {
        let span: Span = Span::from_span(input.as_span());
        Ok(Str{ span, string: String::from(input.as_str().to_owned()) })
    }

    // Expression
    // Climbs operator precedence defined in precedence.rs
    // TODO: Work out how to involve UnaryOperators in precedence
    #[prec_climb(term, PRECCLIMBER)]
    fn expression(left: Term, op: Node, right: Term) -> Result<Term> {
        match op.as_rule() {
            Rule::assignment
                | Rule::logical_or
                | Rule::logical_and
                | Rule::equal
                | Rule::not_equal
                | Rule::greater_than_or_equal
                | Rule::less_than_or_equal
                | Rule::greater_than
                | Rule::less_than
                | Rule::bitwise_xor
                | Rule::bitwise_or
                | Rule::bitwise_and
                | Rule::shift_right
                | Rule::shift_left
                | Rule::plus
                | Rule::minus
                | Rule::multiply
                | Rule::divide
                | Rule::exponent 
            => {
                let span: Span = Span::from_span(op.as_span());
                Ok(Term {
                    node: Box::new(NodeType::BinaryOperator(BinaryOperator {
                        left: left.node,
                        op: Token { value: op.as_str().to_owned(), span },
                        right: right.node,
                        span: Span{ start: left.span.start, end: right.span.end }
                    })),
                    span: Span{ start: left.span.start, end: right.span.end }
                })
            },
            r => Err(op.error(format!("Rule {:?} isn't an operator", r)))?
        }
    }

    // Term, returns wrapper struct for any Node involved in a binary operator
    fn term(input: Node) -> Result<Term> {
        Ok(match_nodes!(input.into_children();
            [expression(expr)] => {
                let node_span: Span = expr.span;
                Term{ node: expr.node, span: node_span }
            },
            [identifier(id)] => {
                let node_span: Span = id.span;
                Term{ node: Box::new(NodeType::Identifier(id)), span: node_span }
            },
            [integer(int)] => {
                let node_span: Span = int.span;
                Term{ node: Box::new(NodeType::Integer(int)), span: node_span }
            },
            [character(c)] => {
                let node_span: Span = c.span;
                Term{ node: Box::new(NodeType::Character(c)), span: node_span }
            },
            [string(s)] => {
                let node_span: Span = s.span;
                Term{ node: Box::new(NodeType::Str(s)), span: node_span }
            },
            [float(float)] => {
                let node_span: Span = float.span;
                Term{ node: Box::new(NodeType::Float(float)), span: node_span }
            },
            [boolean(boolean)] => {
                let node_span: Span = boolean.span;
                Term{ node: Box::new(NodeType::Boolean(boolean)), span: node_span }
            }
        ))
    }

    // Code block
    fn block(input: Node) -> Result<Block> {
        let span: Span = Span::from_span(input.as_span());
        let mut lb: Option<Token> = None;
        let mut stmts: Vec<Box<NodeType>> = vec!();
        let mut rb: Option<Token> = None;

        for n in input.into_children().peekable() {
            match n.as_rule() {
                Rule::lbrace => lb = Some(Self::lbrace(n).unwrap()),
                Rule::rbrace => rb = Some(Self::rbrace(n).unwrap()),

                Rule::expression_statement => stmts.push(Box::new(NodeType::Expression(Self::expression_statement(n).unwrap()))),
                Rule::return_statement => stmts.push(Box::new(NodeType::Return(Self::return_statement(n).unwrap()))),
                Rule::variable_declaration => stmts.push(Box::new(NodeType::VariableDeclaration(Self::variable_declaration(n).unwrap()))),
                Rule::variable_initialisation => stmts.push(Box::new(NodeType::VariableInitialisation(Self::variable_initialisation(n).unwrap()))),
                Rule::function_definition => stmts.push(Box::new(NodeType::FunctionDefinition(Self::function_definition(n).unwrap()))),
                _ => {}
            }
        }

        // Ensure that both brace Tokens have been collected
        assert_ne!(lb.is_none() || rb.is_none(), true);

        Ok(Block{
            lbrace: lb.unwrap(),
            stmts,
            rbrace: rb.unwrap(),
            span 
        })
    }
    
    // A typed parameter
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

    // List of typed parameters, used in Function Signature
    fn parameter_list(input: Node) -> Result<ParameterList> {
        let span: Span = Span::from_span(input.as_span());

        let mut lp: Option<Token> = None;
        let mut params: Vec<(Parameter, Option<Token>)> = vec!();
        let mut rp: Option<Token> = None;

        let mut next_param: Option<Parameter> = None;

        for n in input.into_children().peekable() {
            match n.as_rule() {
                Rule::lparen => {
                    lp = Some(Self::lparen(n).unwrap());
                },
                Rule::parameter => {
                    next_param = Some(Self::parameter(n).unwrap());
                },
                Rule::comma => {
                    params.push((next_param.unwrap(), Some(Self::comma(n).unwrap())));
                    next_param = None;
                },
                Rule::rparen => {
                    if params.len() > 0 {
                        assert_eq!(next_param.is_none(), false);
                        params.push((next_param.unwrap(), None));
                        next_param = None;
                    }
                    
                    rp = Some(Self::rparen(n).unwrap());
                },
                _ => {}
            }
        }

        // Ensure that both parenthesis Tokens are collected
        assert_ne!(lp.is_none() || rp.is_none(), true);

        Ok(ParameterList{
            lparen: lp.unwrap(),
            parameters: params,
            rparen: rp.unwrap(),
            span: span
        })
    }

    // Return type of a function, used in Function Signature
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

    //
    // Statement Nodes
    //

    fn function_definition(input: Node) -> Result<FunctionDefinition> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [proc_token(func_token), identifier(id), parameter_list(params), output(output), block(block)] => {
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

    fn variable_declaration(input: Node) -> Result<VariableDeclaration> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [let_token(let_token), identifier(id), colon(colon), identifier(ty)] => {
                let ty_span: Span = ty.span;
                let ty = TypeSpecifier{ id: ty, span: ty_span };
                
                VariableDeclaration{ let_token, id, colon, ty, span }
            }
        ))
    }

    fn variable_initialisation(input: Node) -> Result<VariableInitialisation> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [let_token(let_token), identifier(id), colon(colon), identifier(ty), equals(equals), expression(expr)] => {
                let ty_span: Span = ty.span;
                let ty = TypeSpecifier{ id: ty, span: ty_span };

                let expr: Box<NodeType> = expr.node;
                VariableInitialisation{ let_token, id, colon, ty: Some(ty), equals, expr: expr, span }
            },
            [let_token(let_token), identifier(id), colon(colon), equals(equals), expression(expr)] => {
                let expr: Box<NodeType> = expr.node;
                VariableInitialisation{ let_token, id, colon, ty: None, equals, expr: expr, span }
            },
        ))
    }

    fn return_statement(input: Node) -> Result<ReturnStatement> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [return_token(ret), expression(expr)] => ReturnStatement{ return_token: ret, expr: expr.node, span }
        ))
    }

    fn expression_statement(input: Node) -> Result<ExpressionStatement> {
        let span: Span = Span::from_span(input.as_span());
        Ok(match_nodes!(input.into_children();
            [expression(expr)] => ExpressionStatement{ expr: expr.node, span }
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
        let mut statements: Vec<Box<NodeType>> = Vec::new();
        for node in nodes {
            let rule = node.as_rule();

            // Match and build all Statements
            match rule {
                Rule::EOI => {},
                Rule::expression_statement => statements.push(Box::new(NodeType::Expression(Self::expression_statement(node)?))),
                Rule::return_statement => statements.push(Box::new(NodeType::Return(Self::return_statement(node)?))),
                Rule::variable_declaration => statements.push(Box::new(NodeType::VariableDeclaration(Self::variable_declaration(node)?))),
                Rule::variable_initialisation => statements.push(Box::new(NodeType::VariableInitialisation(Self::variable_initialisation(node)?))),
                Rule::function_definition => statements.push(Box::new(NodeType::FunctionDefinition(Self::function_definition(node)?))),
                // Rule::import_statement => statements.push(Box::new(Self::import_statement(node)?)),
                // Rule::use_statement => statements.push(Box::new(Self::use_statement(node)?)),
                _ => {}
            }
        }

        Ok(File{ stmts: statements, span: Some(span) })
    }
}

#[derive(Copy, Clone)]
pub struct LineColumn {
    line: usize,
    column: usize
}

impl LineColumn {
    pub fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "LineColumn".color("green"));
        let indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{}: {}", output, indent, "├──", "line".color("blue"), self.line);
        output = format!("{}\n{}{}{}: {}", output, indent, "└──", "column".color("blue"), self.column);
        output
    }
}

// Used to store location of AST Nodes and Tokens
#[derive(Copy, Clone)]
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

    pub fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Span".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        let start_tree: String = self.start.display_tree(&mut indent, false);
        output = format!("{}\n{}{}{}: \n{}", output, indent, "├──", "start".color("blue"), start_tree);

        let end_tree: String = self.end.display_tree(&mut indent, true);
        output = format!("{}\n{}{}{}: \n{}", output, indent, "├──", "end".color("blue"), end_tree);
        output
    }
}

// Parse worker function
pub fn parse_gecko(input_str: &str) -> Result<File> {
    let inputs = GeckoParser::parse_with_userdata(Rule::file, input_str, ())?;
    let input = inputs.single()?;

    GeckoParser::file(input)
}