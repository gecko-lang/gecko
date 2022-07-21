use crate::ast::Span;
use crate::expression::Expression;
use crate::node::ASTNode;

use crate::colored::*;

#[derive(Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span
}

impl Expression for Identifier {}

impl ASTNode for Identifier {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Identifier".color("green"));
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}{}{}: {}\n", output, indent, "├──", "name".color("blue"), self.name);
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}
