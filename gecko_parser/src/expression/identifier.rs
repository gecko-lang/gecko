use crate::ast::Span;
use crate::expression::Expression;

pub struct Identifier {
    pub name: String,
    pub span: Span
}

impl Expression for Identifier {
    fn display(&self) -> String {
        format!("{}", self.name)
    }
}