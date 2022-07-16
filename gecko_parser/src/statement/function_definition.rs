use crate::ast::Span;
use crate::statement::Statement;
use crate::node::{ParameterList, Output, Block};
use crate::Token;

use crate::expression::Identifier;
use crate::expression::Expression;
// use crate::expression::IdentifierList;
// use crate::expression::TypeSpecifier;
// use crate::statement::Block;

//#[derive(Debug)]
pub struct FunctionDefinition {
    
    // pub attributes: Vec<Attribute>,
    pub sig: Signature,
    pub block: Block,
    pub span: Span
}

impl Statement for FunctionDefinition {
    fn display(&self, indent: i16) -> String {
        format!(
            "{} {}",
            self.sig.display(indent + 1),
            self.block.display(indent + 1)
            
        )
    }
}

#[derive(Debug)]
pub struct Signature {
    pub func_token: Token,
    pub id: Identifier,
    // pub generics: Vec<Generic>,
    pub params: ParameterList,
    pub output: Output,
    pub span: Span,
}

impl Signature {
    pub fn display(&self, indent: i16) -> String {
        format!("{} {}{} {}",
            self.func_token.display(indent + 1),
            self.id.display(indent + 1),
            self.params.display(indent + 1),
            self.output.display(indent + 1)
        )
    }
}