use crate::ast::Span;
use crate::statement::Statement;
use crate::node::Node;

use crate::colored::*;

pub struct ExpressionStatement {
    pub expr: Box<dyn Node>,
    pub span: Span
}

impl Statement for ExpressionStatement {}

impl Node for ExpressionStatement {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "ExpressionStatement".color("red").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}\n", output, self.expr.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}