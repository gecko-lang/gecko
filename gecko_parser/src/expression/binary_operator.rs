
use crate::{
    ast::Span,
    expression::Expression,
    node::Node,
    Token
};

use crate::colored::*;

pub struct BinaryOperator {
    pub left: Box<dyn Node>,
    pub op: Token,
    pub right: Box<dyn Node>,
    pub span: Span
}

impl Expression for BinaryOperator {}

impl Node for BinaryOperator {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "BinaryOperator".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{}:\n", output, indent, "├──", "left".color("blue"));
        output = format!("{}{}", output, self.left.display_tree(&mut indent, false));

        output = format!("{}\n{}{}{}:\n", output, indent, "├──", "op".color("blue"));
        output = format!("{}{}", output, self.op.display_tree(&mut indent, false));

        output = format!("{}\n{}{}{}:\n", output, indent, "├──", "right".color("blue"));
        output = format!("{}{}", output, self.right.display_tree(&mut indent, true));
        output
    }
}
