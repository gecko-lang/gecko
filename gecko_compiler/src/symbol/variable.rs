use gecko_parser::expression::Identifier;

use crate::Type;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Variable {
    init: bool,

    pub id: Identifier,
    pub ty: Option<Type>,
}

#[allow(dead_code)]
impl Variable {
    pub fn new(init: bool, id: Identifier, ty: Type) -> Self {
        Variable{ init, id, ty: Some(ty) }
    }

    pub fn initialise(&mut self) {
        self.init = true;
    }
    pub fn is_initialised(&self) -> bool {
        self.init
    }
}