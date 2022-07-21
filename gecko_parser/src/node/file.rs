
use crate::ast::Span;
use crate::node::{ASTNode, NodeType};

use crate::colored::*;

pub struct File {
    pub stmts: Vec<Box<NodeType>>,
    pub span: Option<Span>,
}

impl ASTNode for File {
    fn display_tree(&self, indent: &mut String, _is_last: bool) -> String {
        let mut output: String = format!("{}", "File\n".color("yellow").dimmed());
        let mut indent: String = (*indent).clone();

        for stmt in &self.stmts {
            output = format!("{}{}\n", output, stmt.display_tree(&mut indent, false));
        }

        output = format!("{}{}", output, self.span.unwrap().display_tree(&mut indent, true));
        output
    }
}