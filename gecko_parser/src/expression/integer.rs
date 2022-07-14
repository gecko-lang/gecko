use crate::ast::Span;
use crate::expression::Expression;

pub struct Integer {
    pub span: Span,
    pub value: String
}

impl Expression for Integer {
    fn display(&self) -> String {
        String::from(self.value.to_string())
    }
}