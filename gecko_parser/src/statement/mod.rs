pub mod function_definition;

pub use function_definition::FunctionDefinition;
pub use function_definition::Signature;

pub trait Statement {
    fn display(&self) -> String;
}