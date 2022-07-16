
use crate::ast::Span;
use crate::Token;
use crate::node::{TypeSpecifier};

//#[derive(Debug)]
pub struct Output {
    pub rarrow: Token,
    pub ty: TypeSpecifier,
    pub span: Span
}

impl Output {
    pub fn display(&self, indent: i16) -> String {
        format!("{} {}",
            self.rarrow.display(indent + 1), 
            self.ty.display(indent + 1)
        )
    }
}