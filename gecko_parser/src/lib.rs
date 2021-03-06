#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate enum_dispatch;

extern crate colored;

pub mod ast;
pub mod error;
pub mod grammar;
pub mod statement;
pub mod token;
pub mod expression;
pub mod node;
pub mod precedence;

pub use token::Token;