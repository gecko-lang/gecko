use crate::ast::Span;
use crate::statement::Statement;
use crate::node::{ParameterList, Output, Block, Node};
use crate::Token;

use crate::expression::Identifier;
// use crate::expression::IdentifierList;
// use crate::expression::TypeSpecifier;
// use crate::statement::Block;

use crate::colored::*;

pub struct FunctionDefinition {
    
    // pub attributes: Vec<Attribute>,
    pub sig: Signature,
    pub block: Block,
    pub span: Span
}

impl Statement for FunctionDefinition {}

impl Node for FunctionDefinition {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "FunctionDefinition".color("red").dimmed());
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}", output, self.sig.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.block.display_tree(&mut indent, false));
        output = format!("{}\n{}", output, self.span.display_tree(&mut indent, true));
        output
    }
}

pub struct Signature {
    pub func_token: Token,
    pub id: Identifier,
    // pub generics: Vec<Generic>,
    pub params: ParameterList,
    pub output: Output,
    pub span: Span,
}

impl Node for Signature {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String {
        let marker = if is_last { String::from("└──") } else { String::from("├──") };
        let mut output: String = format!("{}{}{}\n", indent, marker, "Signature");
        let mut indent: String = if is_last { (*indent).clone() + "    " } else { (*indent).clone() + "│   " };

        output = format!("{}{}\n", output, self.func_token.display_tree(&mut indent, false));
        output = format!("{}{}", output, self.id.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.params.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.output.display_tree(&mut indent, false));
        output = format!("{}{}\n", output, self.span.display_tree(&mut indent, true));
        output
    }
}