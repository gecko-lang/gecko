use cranelift::prelude::*;
use cranelift_codegen::{
    ir::{AbiParam, Signature},
    isa::CallConv
};

use crate::{
    File,
    symbol::SymbolTable,
    node::NodeType,
    tree_type::FundamentalType
};

impl FundamentalType {
    fn to_ir_type(&self) -> Type {
        match *self {
            FundamentalType::Integer => types::I32,
            FundamentalType::Boolean => types::B1,
            FundamentalType::Character => types::I8,
            FundamentalType::Float => types::F32,
            _ => panic!("No conversion of Gecko type to IR type")
        }
    }
}

pub fn generate_ir(tree: &File, symbol_table: SymbolTable) {
    for stmt in &tree.stmts {
        match &**stmt {
            NodeType::FunctionDefinition(function_definition) => {
                let fn_sig = &function_definition.sig;
                let fn_name = &fn_sig.id.name;
                let symbol =  symbol_table.symbols.get(fn_name).unwrap();

                let mut sig = Signature::new(CallConv::SystemV);
                //sig.returns.push(AbiParam::new())
            },
            _ => panic!("{}", "This statement cannot be in the global scope")
        }
    }
}