
use crate::ast::Span;
use crate::statement::Statement;

//#[derive(Debug)]
pub struct File {
    pub stmts: Vec<Box<dyn Statement>>,
    pub span: Option<Span>,
}

impl File {
    pub fn display(&self, indent: i16) -> String {
        let mut stmts = vec![String::from("File:")];
        for stmt in &self.stmts {
            stmts.push(stmt.display(indent));
            stmts.push(String::from("\n"));
        }
        stmts.concat()
    }
}