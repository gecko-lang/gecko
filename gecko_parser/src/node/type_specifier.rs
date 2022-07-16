
use crate::ast::Span;
use crate::expression::Identifier;

pub struct TypeSpecifier {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub span: Span
}