use crate::ast::Span;

pub struct Token {
    pub span: Span,
    pub value: String
}

impl Token {
    fn display(&self) -> String {
        return String::from(self.value.to_string());
    } 
}