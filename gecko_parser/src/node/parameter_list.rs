
use crate::ast::Span;
use crate::node::{Parameter, ASTNode};
use crate::Token;

use crate::colored::*;

pub struct ParameterList {
    pub lparen: Token,
    pub parameters: Vec<(Parameter, Option<Token>)>,
    pub rparen: Token,
    pub span: Span
}

impl ASTNode for ParameterList {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "ParameterList".color("yellow").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}\n", output, self.lparen.display_tree(&mut indent, false));
        //output = format!("{}\n{}", output, self.parameters.display_tree(indent, false));
        output = format!("{}{}\n", output, self.rparen.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}