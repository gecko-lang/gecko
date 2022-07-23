
use crate::ast::Span;
use crate::expression::Identifier;
use crate::node::{TypeSpecifier, ASTNode};
use crate::Token;

use crate::colored::*;

pub struct Parameter {
    // pub attrs: Vec<Attribute>; 
    pub id: Identifier,
    pub colon_token: Token,
    pub ty: TypeSpecifier,
    pub span: Span,
}

impl ASTNode for Parameter {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "Parameter".color("yellow").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}\n", output, self.id.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.colon_token.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.ty.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}