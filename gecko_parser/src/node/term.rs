
use crate::ast::Span;
use crate::node::Node;
use crate::expression::Expression;

use crate::colored::*;

pub struct Term {
    // pub attrs: Vec<Attribute>; 
    pub node: Box<dyn Node>,
    pub span: Span,
}

impl Expression for Term {}

impl Node for Term {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}", indent, marker, "Term".color("yellow").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}\n{}", output, self.node.display_tree(&mut indent, false));
        output = format!("{}\n{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}