use gecko_parser::expression::Identifier;

use crate::Type;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Variable {
    id: Identifier,
    ty: Option<Type>
}

#[allow(dead_code)]
impl Variable {
    fn is_initialised(&self) -> bool {
        !self.ty.is_none()
    }
}