use std::collections::HashMap;

pub mod variable;
pub mod function;
pub use variable::*;
pub use function::*;

use gecko_parser::{
    expression::Identifier,
    statement::function_definition::Signature,
};
use crate::{
    Type,
    error::TypeError
};

#[derive(Clone)]
pub enum Symbol {
    Variable(Variable),
    Function(Function)
}

#[derive(Clone)]
pub struct SymbolTable {
    // Need to use a map instead of a vector for symbol names
    pub symbols: HashMap<String, Symbol>
}

impl SymbolTable {
    pub fn declare_variable(&mut self, id: &Identifier, ty: Type) {
        let symbol: Symbol = Symbol::Variable(Variable::new(false, id.clone(), ty));
        self.symbols.insert(id.name.clone(), symbol);
    }
    pub fn initialise_variable(&mut self, id: &Identifier, ty: Type) {
        let symbol: Symbol = Symbol::Variable(Variable::new(true, id.clone(), ty));
        self.symbols.insert(id.name.clone(), symbol);
    }

    // Checks if variable exists and is initialised
    pub fn variable_type(&self, id: &Identifier) -> Result<Option<Type>, TypeError> {
        for (name, symbol) in &self.symbols {
            if name.as_str() == id.name {   
                match symbol {
                    Symbol::Variable(variable) => {
                        if !variable.is_initialised() {
                            // Attempt to use uninitialised variable
                            return Err(TypeError{ text: format!("Attempt to access uninitialised variable '{}'", name)});
                        }
                        return Ok(variable.ty.clone());
                    },
                    _ => return Err(TypeError{ text: format!("'{}', is not a variable", id.name)})
                }
            }
        }

        Err(TypeError{ text: format!("The variable '{}' does not exist", id.name)})
    }

    pub fn define_function(&mut self, id: &Identifier, params: Vec<(String, Type)>, output: Type) {
        let symbol: Symbol = Symbol::Function(Function::new(true, id.clone(), params, output));
        self.symbols.insert(id.name.clone(), symbol);
    }
}