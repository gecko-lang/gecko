pub mod function_definition;

pub use function_definition::FunctionDefinition;

pub trait Statement {
    fn display(&self) -> String;
}