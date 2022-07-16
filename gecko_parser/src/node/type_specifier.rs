
use crate::ast::Span;
use crate::expression::{Identifier, Expression};

pub struct TypeSpecifier {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub span: Span
}

impl TypeSpecifier {
    pub fn display(&self) -> String {
        self.id.display()
    }
}