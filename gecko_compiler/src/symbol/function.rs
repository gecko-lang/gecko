
use gecko_parser::expression::Identifier;

use crate::Type;

#[derive(Clone)]
pub struct Function {
    id: Identifier,
    params: Vec<(String, Type)>,
    output: Type,

    body: bool,
}

impl Function {
    pub fn new(body: bool, id: Identifier, params: Vec<(String, Type)>, output: Type) -> Self {
        Self{ body, id, output, params }
    }

    pub fn define(&mut self) {
        self.body = true;
    }
    pub fn has_body(&self) -> bool {
        self.body
    }
}