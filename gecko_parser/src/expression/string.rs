use crate::ast::Span;
use crate::expression::Expression;
use crate::node::ASTNode;

use crate::colored::*;

pub struct Str {
    pub string: String,
    pub span: Span
}

impl Expression for Str {}

impl ASTNode for Str {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "String".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{} {}", output, indent, "├──", "value:", self.string);
        output = format!("{}{}\n", output, self.span.display_tree(&mut indent, true));
        output
    }
}