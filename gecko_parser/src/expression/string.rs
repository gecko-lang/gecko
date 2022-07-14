use crate::ast::Span;
use crate::expression::Expression;

pub struct Str {
    pub span: Span,
    pub string: String
}

impl Expression for Str {
    fn display(&self) -> String {
        String::from(self.string.to_string())
    }
}