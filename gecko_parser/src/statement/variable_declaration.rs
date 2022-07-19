
use crate::ast::Span;
use crate::statement::Statement;
use crate::node::{Node, TypeSpecifier};
use crate::expression::Identifier;
use crate::Token;

use crate::colored::*;

pub struct VariableDeclaration {
    pub let_token: Token,
    pub id: Identifier,
    pub colon: Token,
    pub ty: TypeSpecifier,
    pub span: Span
}

impl Statement for VariableDeclaration {}

impl Node for VariableDeclaration {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "VariableDeclaration".color("red").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}\n", output, self.let_token.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.id.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.colon.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.ty.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}