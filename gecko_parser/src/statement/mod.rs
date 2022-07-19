
pub mod function_definition;
pub use function_definition::FunctionDefinition;
pub use function_definition::Signature;

pub mod expression_statement;
pub use expression_statement::ExpressionStatement;

 // trait that all AST Nodes that are Statements implement
pub trait Statement {

}