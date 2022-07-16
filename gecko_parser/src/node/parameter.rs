
use crate::ast::Span;
use crate::expression::Identifier;
use crate::node::TypeSpecifier;
use crate::Token;

pub struct Parameter {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub colon_token: Token,
    pub ty: TypeSpecifier,
    pub span: Span,
}