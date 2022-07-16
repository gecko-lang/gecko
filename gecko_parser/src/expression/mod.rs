pub mod identifier;
pub mod integer;
pub mod string;

pub use identifier::Identifier;
pub use integer::Integer;
pub use string::Str;

pub trait Expression {
    fn display(&self, indent: i16) -> String;
}