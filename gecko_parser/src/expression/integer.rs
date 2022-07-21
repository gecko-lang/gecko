use crate::ast::Span;
use crate::expression::Expression;
use crate::node::ASTNode;

use crate::colored::*;

pub struct Integer {
    pub value: i128,
    pub span: Span
}

impl Expression for Integer {}

impl ASTNode for Integer {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Integer".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{} {}\n", output, indent, "├──", "value:", self.value);
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}