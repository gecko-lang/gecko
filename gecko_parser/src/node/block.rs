
use crate::ast::Span;
use crate::Token;
// use crate::statement::Statement;
use crate::node::{ASTNode, NodeType};

use crate::colored::*;

pub struct Block {
    // pub attrs: Vec<Attribute>; 
    pub lbrace: Token,
    pub stmts: Vec<Box<NodeType>>,
    pub rbrace: Token,
    pub span: Span,
}

impl ASTNode for Block {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Block".color("yellow").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}", output, self.lbrace.display_tree(&mut indent, false));

        for stmt in &self.stmts {
            output = format!("{}\n{}", output, stmt.display_tree(&mut indent, false));
        }

        output = format!("{}\n{}", output, self.rbrace.display_tree(&mut indent, false));
        output = format!("{}\n{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}