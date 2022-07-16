
use crate::ast::Span;
use crate::expression::{Identifier, Expression};
use crate::node::TypeSpecifier;
use crate::Token;

pub struct Parameter {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub colon_token: Token,
    pub ty: TypeSpecifier,
    pub span: Span,
}

impl Parameter {
    pub fn display(&self) -> String {
        format!("{}{} {}",
            self.id.display(),
            self.colon_token.display(),
            self.ty.display()
        )
    }
}