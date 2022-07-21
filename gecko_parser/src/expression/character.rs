use crate::ast::Span;
use crate::expression::Expression;
use crate::node::ASTNode;
use crate::Token;

use crate::colored::*;

pub struct Character {
    pub lquote: Token,
    pub value: u8,
    pub rquote: Token,
    pub span: Span
}

impl Expression for Character {}

impl ASTNode for Character {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Character".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}", output, self.lquote.display_tree(&mut indent, false));
        output = format!("{}\n{}{}{} {}\n", output, indent, "├──", "value:", self.value);
        output = format!("{}{}", output, self.rquote.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}