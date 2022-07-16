use crate::ast::Span;
use crate::node::Parameter;
use crate::Token;

pub struct ParameterList {
    pub lparen: Token,
    pub parameters: Vec<(Parameter, Option<Token>)>,
    pub rparen: Token,
    pub span: Span
}