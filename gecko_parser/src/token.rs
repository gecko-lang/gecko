use crate::ast::Span;

pub struct Token {
    pub span: Span,
    pub value: String
}

impl Token {
    pub fn display(&self, indent: i16) -> String {
        return String::from(self.value.to_string());
    } 
}