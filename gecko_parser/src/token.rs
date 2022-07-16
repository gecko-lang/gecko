use crate::ast::Span;
use crate::node::Node;

use crate::colored::*;

pub struct Token {
    pub value: String,
    pub span: Span
}

impl Node for Token {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Token".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{}: {}\n", output, indent, "├──", "value".color("blue"), self.value);
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    } 
}