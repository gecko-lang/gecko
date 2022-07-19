
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

 // Trait which all Gecko AST nodes implement
pub trait Node {
    fn display_tree(&self, indent: &mut String, is_last: bool) -> String;
}