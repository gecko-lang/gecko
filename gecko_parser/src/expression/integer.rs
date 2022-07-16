use crate::ast::Span;
use crate::expression::Expression;

//#[derive(Debug)]
pub struct Integer {
    pub value: String,
    pub span: Span
}

impl Expression for Integer {
    fn display(&self, indent: i16) -> String {
        String::from(self.value.to_string())
    }
}