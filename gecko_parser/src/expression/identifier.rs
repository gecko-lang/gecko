use crate::ast::Span;
use crate::expression::Expression;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub span: Span
}

impl Expression for Identifier {
    fn display(&self, indent: i16) -> String {
        self.name.clone()
    }
}