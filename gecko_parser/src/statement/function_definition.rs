use crate::ast::Span;
use crate::statement::Statement;

use crate::expression::Identifier;
use crate::expression::IdentifierList;
use crate::expression::TypeSpecifier;
use crate::statement::Block;

pub struct FunctionDefinition {
    pub span: Span,
    pub id: Identifier,
    pub param_list: IdentifierList,
    pub return_type: TypeSpecifier,
    pub block: Block
}

impl Statement for FunctionDefinition {
    fn display(&self) -> String {
        format!("func {}", self.id.name)
    }
}