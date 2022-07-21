
pub mod identifier;
pub mod boolean;
pub mod character;
pub mod integer;
pub mod float;
pub mod string;

pub use identifier::Identifier;
pub use boolean::Boolean;
pub use character::Character;
pub use integer::Integer;
pub use float::Float;
pub use string::Str;

pub mod binary_operator;
pub use binary_operator::BinaryOperator;

 // Expression trade for all nodes that are expressions
pub trait Expression {

}