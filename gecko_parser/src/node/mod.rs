pub mod block;
pub use block::Block;

pub mod parameter;
pub use parameter::Parameter;

pub mod parameter_list;
pub use parameter_list::ParameterList;

pub mod type_specifier;
pub use type_specifier::TypeSpecifier;

pub mod output;
pub use output::Output;

pub mod file;
pub use file::File;

pub mod term;
pub use term::Term;

pub use crate::colored::*;

use crate::{
    Token,
    expression,
    node,
    statement,
};

#[enum_dispatch]
pub enum NodeType {
    Token(Token),

    BinaryOperator(expression::BinaryOperator),
    Boolean(expression::Boolean),
    Character(expression::Character),
    Float(expression::Float),
    Integer(expression::Integer),
    Identifier(expression::Identifier),
    Str(expression::Str),
    
    Block(node::Block),
    File(node::File),
    Output(node::Output),
    ParameterList(node::ParameterList),
    Parameter(node::Parameter),
    Term(node::Term),
    TypeSpecifier(node::TypeSpecifier),
    Signature(statement::function_definition::Signature),

    Expression(statement::ExpressionStatement),
    FunctionDefinition(statement::FunctionDefinition),
    Return(statement::ReturnStatement),
    VariableDeclaration(statement::VariableDeclaration),
    VariableInitialisation(statement::VariableInitialisation),
}

// Trait which all Gecko AST nodes implement
#[enum_dispatch(NodeType)]
pub trait ASTNode {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String;
}