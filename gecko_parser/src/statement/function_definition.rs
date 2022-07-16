use crate::ast::Span;
use crate::statement::Statement;
use crate::node::{ParameterList, Output, Block};
use crate::Token;

use crate::expression::Identifier;
// use crate::expression::IdentifierList;
// use crate::expression::TypeSpecifier;
// use crate::statement::Block;

pub struct FunctionDefinition {
    
    // pub attributes: Vec<Attribute>,
    pub sig: Signature,
    pub block: Block,
    pub span: Span
}

impl Statement for FunctionDefinition {
    fn display(&self) -> String {
        format!("func {}", self.sig.id.name)
    }
}

pub struct Signature {
    pub func_token: Token,
    pub id: Identifier,
    // pub generics: Vec<Generic>,
    pub params: ParameterList,
    pub output: Output,
    pub span: Span,
}