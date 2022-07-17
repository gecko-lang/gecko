
pub mod identifier;
pub mod integer;
pub mod string;

pub use identifier::Identifier;
pub use integer::Integer;
pub use string::Str;

pub mod binary_operator;
pub use binary_operator::BinaryOperator;

pub trait Expression {

}