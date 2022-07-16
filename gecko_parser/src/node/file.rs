
use crate::ast::Span;
use crate::statement::Statement;

pub struct File {
    pub stmts: Vec<Box<dyn Statement>>,
    pub span: Option<Span>,
}