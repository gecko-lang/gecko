use crate::ast::Span;
use crate::expression::Expression;

pub struct Identifier {
    pub span: Span,
    pub name: String
}

impl Expression for Identifier {
    fn display(&self) -> String {
        format!("{}", self.name)
    }
}