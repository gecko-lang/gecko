
use crate::ast::Span;
use crate::Token;
use crate::node::{TypeSpecifier};

pub struct Output {
    pub rarrow: Token,
    pub ty: TypeSpecifier,
    pub span: Span
}