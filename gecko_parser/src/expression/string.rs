use crate::ast::Span;
use crate::expression::Expression;

pub struct Str {
    pub string: String,
    pub span: Span
}

impl Expression for Str {
    fn display(&self) -> String {
        String::from(self.string.to_string())
    }
}