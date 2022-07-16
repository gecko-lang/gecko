
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