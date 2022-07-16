
use crate::ast::Span;
use crate::Token;
use crate::statement::Statement;

pub struct Block {
    // pub attrs: Vec<Attribute>; 
    pub lbrace: Token,
    pub stmts: Vec<Box<dyn Statement>>,
    pub rbrace: Token,
    pub span: Span,
}

impl Block {
    pub fn display(&self) -> String {
        // TODO: Display all statements within a block
        String::from("{ . . . }")
    }
}