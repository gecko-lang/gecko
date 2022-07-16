use crate::ast::Span;
use crate::expression::Expression;

pub struct Integer {
    pub value: String,
    pub span: Span
}

impl Expression for Integer {
    fn display(&self) -> String {
        String::from(self.value.to_string())
    }
}