//#[allow(dead_code)]
use std::{collections::HashMap, fmt};

use crate::{
    symbol::SymbolTable
};

use gecko_parser::{
    expression,
    node,
    statement,
    node::{
        NodeType
    },
    Token
};

use crate::error::{TypeError};

#[derive(Debug, PartialEq, Clone)]
pub enum FundamentalType {
    Boolean,
    Character,
    Integer,
    Float,
    String,
    DefinedType
}

impl fmt::Display for FundamentalType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(match self {
            FundamentalType::Boolean => "bool",
            FundamentalType::Character => "char",
            FundamentalType::Integer => "int",
            FundamentalType::Float => "float",
            FundamentalType::String => "str",
            FundamentalType::DefinedType => "defined type"
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    ty: FundamentalType
}

impl Type {
    fn from_id(id: &expression::Identifier) -> Self {
        Type::from_string(&id.name)
    }
    fn from_string(string: &String) -> Self {
        match string.as_str() {
            "bool" => Type{ ty: FundamentalType::Boolean },
            "char" => Type{ ty: FundamentalType::Character },
            "int" => Type{ ty: FundamentalType::Integer },
            "float" => Type{ ty: FundamentalType::Float },
            "str" => Type{ ty: FundamentalType::String },
            _ => Type{ ty: FundamentalType::DefinedType },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("{}", self.ty).as_str())
    }
}

pub trait TypeCheck {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError>;
}

impl TypeCheck for NodeType {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        use NodeType::*;
        match self {
            Token(token) => token.check(symbol_table),

            BinaryOperator(binary_operator) => binary_operator.check(symbol_table),
            Boolean(boolean) => boolean.check(symbol_table),
            Character(character) => character.check(symbol_table),
            Float(float) => float.check(symbol_table),
            Integer(integer) => integer.check(symbol_table),
            Identifier(identifier) => identifier.check(symbol_table),
            Str(string) => string.check(symbol_table),

            Block(block) => block.check(symbol_table),
            File(file) => file.check(symbol_table),
            Output(output) => output.check(symbol_table),
            ParameterList(parameter_list) => parameter_list.check(symbol_table),
            Parameter(parameter) => parameter.check(symbol_table),
            Term(term) => term.check(symbol_table),
            TypeSpecifier(type_specifier) => type_specifier.check(symbol_table),

            Expression(expression) => expression.check(symbol_table),
            Signature(signature) => signature.check(symbol_table),
            FunctionDefinition(function_definition) => function_definition.check(symbol_table),
            Return(return_statement) => return_statement.check(symbol_table),
            VariableDeclaration(variable_declaration) => variable_declaration.check(symbol_table),
            VariableInitialisation(variable_initialisation) => variable_initialisation.check(symbol_table)
        }
    }
}

impl TypeCheck for Token {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(None)
    }
}

impl TypeCheck for expression::Boolean {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Boolean }))
    }
}

impl TypeCheck for expression::Character {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Character }))
    }
}

impl TypeCheck for expression::Float {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Float }))
    }
}

impl TypeCheck for expression::Integer {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Integer }))
    }
}

impl TypeCheck for expression::Str {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::String }))
    }
}

impl TypeCheck for expression::Identifier {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        symbol_table.variable_type(&self)
    }
}

pub fn binary_operator_prec(prec: Vec<FundamentalType>, left: &Type, right: &Type) -> Result<Option<Type>, TypeError> {
    if prec.contains(&left.ty) && prec.contains(&right.ty) {
        let left_prec = prec
                        .iter()
                        .position(|t| t == &left.ty);
        let right_prec = prec
                        .iter()
                        .position(|t| t == &right.ty);

        if left_prec < right_prec {
            return Ok(Some((*left).clone()));
        }
        else if left_prec > right_prec {
            return Ok(Some((*right).clone()));
        }
        else {
            return Ok(Some((*left).clone()));
        }
    }

    return Ok(None)
}

impl TypeCheck for expression::BinaryOperator {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        let left = match self.left.check(symbol_table) {
                Ok(l) => l,
                Err(error) => panic!("{}", error)
            }
            .unwrap();
        let right: Type;
        if self.op.value.as_str() == "as" {
            use NodeType::*;
            match &*self.right {
                Identifier(id) => {
                    right = Type::from_id(&id);
                },
                _ => return Err(TypeError{ text: format!("Type specifier expected after 'as'") })
            }
            
        } else {
            right = match self.right.check(symbol_table) {
                Ok(r) => r,
                Err(error) => panic!("{}", error)
            }
            .unwrap();
        }

        match self.op.value.as_str() {
            "+" | "-" => {
                let prec = vec![
                    FundamentalType::Float,
                    FundamentalType::Integer,
                    FundamentalType::Character,
                ];

                let prec_type = binary_operator_prec(prec, &left, &right);
                if !prec_type.as_ref().unwrap().is_none() {
                    return prec_type;
                }

                if self.op.value == "+" {
                    if left.ty == FundamentalType::String || right.ty == FundamentalType::String {
                        return Ok(Some(Type{ ty: FundamentalType::String }));
                    }
                }

                Err(TypeError{ text: format!("Invalid operand type for operator '{}'", self.op.value) })
            },
            "*" | "/" | ">" | "<" | ">=" | "<=" => {
                let prec = vec![
                    FundamentalType::Float,
                    FundamentalType::Integer,
                    FundamentalType::Character,
                ];

                let prec_type = binary_operator_prec(prec, &left, &right);
                if !prec_type.as_ref().unwrap().is_none() {
                    return prec_type;
                }

                if self.op.value == "*" {
                    if left.ty == FundamentalType::String || right.ty == FundamentalType::String {
                        return Ok(Some(Type{ ty: FundamentalType::String }));
                    }
                }

                Err(TypeError{ text: format!("Invalid operand type for operator '{}'", self.op.value) })
            },
            "&&" | "||" | "==" | "!=" => {
                if left.ty == FundamentalType::Boolean && right.ty == FundamentalType::Boolean {
                    return Ok(Some(Type{ ty: FundamentalType::Boolean })); 
                }

                Err(TypeError{ text: format!("Invalid operand type for operator '{}'", self.op.value) })
            }
            "as" => {
                Ok(Some(right))
            },
            _ => {
                let prec = vec![
                    FundamentalType::Float,
                    FundamentalType::Integer,
                    FundamentalType::Character,
                ];

                let prec_type = binary_operator_prec(prec, &left, &right);
                if !prec_type.as_ref().unwrap().is_none() {
                    return prec_type;
                }

                Err(TypeError{ text: format!("Invalid operand type for operator '{}'", self.op.value) })
            }
        }
    }
}

impl TypeCheck for node::Block {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        for stmt in &self.stmts {
            stmt.check(symbol_table)
                .unwrap();
        }
        Ok(None)
    }
}

impl TypeCheck for node::File {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        for stmt in &self.stmts {
            stmt.check(symbol_table)
                .unwrap();   
        }
        Ok(None)
    }
}

impl TypeCheck for node::Output {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        self.ty.check(symbol_table)
            .unwrap();
        Ok(None)
    }
}

impl TypeCheck for node::ParameterList {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        for (param, _) in &self.parameters {
            param.check(symbol_table)
                .unwrap();
        }
        Ok(None)
    }
}

impl TypeCheck for node::Parameter {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        let param_ty = self.ty.check(symbol_table)
            .unwrap()
            .unwrap();
        symbol_table.initialise_variable(&self.id, param_ty);
        Ok(None)
    }
}

impl TypeCheck for node::Term {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        self.node.check(symbol_table)
    }
}

impl TypeCheck for node::TypeSpecifier {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type::from_id(&self.id)))
    }
}

impl TypeCheck for statement::ExpressionStatement {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        self.expr.check(symbol_table)
            .unwrap();
        Ok(None)
    }
}

impl TypeCheck for statement::function_definition::Signature {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        self.params.check(symbol_table)
            .unwrap();
        self.output.check(symbol_table)
            .unwrap();
        Ok(None)
    }
}

impl TypeCheck for statement::FunctionDefinition {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        let symbol_table: &mut SymbolTable = &mut symbol_table.clone();

        self.sig.check(symbol_table)
            .unwrap();
        self.block.check(symbol_table)
            .unwrap();
        Ok(None)
    }
}

impl TypeCheck for statement::ReturnStatement {
    fn check(&self, _symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError{ text: format!("Not yet implemented") })
    }
}

impl TypeCheck for statement::VariableDeclaration {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        let ty: Type = self.ty.check(symbol_table)
            .unwrap()
            .unwrap();
        symbol_table.declare_variable(&self.id, ty);
        Ok(None)
    }
}

impl TypeCheck for statement::VariableInitialisation {
    fn check(&self, symbol_table: &mut SymbolTable) -> Result<Option<Type>, TypeError> {
        let spec_ty = &self.ty.as_ref();
        let ty: Type;
        if !spec_ty.is_none() {
            ty = spec_ty.unwrap()
                .check(symbol_table)
                .unwrap()
                .unwrap();
            let val_ty: Type = self.expr.check(symbol_table)
                .unwrap()
                .unwrap();
            if ty != val_ty {
                return Err(TypeError{ text: format!("Type mismatch. Expected '{}', got '{}'", ty, val_ty) });
            }
        }
        else {
            ty = self.expr.check(symbol_table)
                .unwrap()
                .unwrap();
        }
        symbol_table.initialise_variable(&self.id, ty.clone());
        Ok(None)
    }
}

pub fn annotate_file(tree: &node::File) -> Result<Option<node::File>, TypeError> {
    let mut symbol_table: &mut SymbolTable = &mut SymbolTable{ symbols: HashMap::new() };
    
    tree.check(&mut symbol_table)
        .unwrap();
    Ok(None)    // Have to work out how to go about annotating the original AST and returning that
                // instead of just checking types 
}