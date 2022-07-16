
use crate::ast::Span;
use crate::expression::Identifier;
use crate::node::Node;

use crate::colored::*;

pub struct TypeSpecifier {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub span: Span
}

impl Node for TypeSpecifier {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "TypeSpecifier".color("yellow").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}", output, self.id.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}