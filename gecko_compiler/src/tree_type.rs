//#[allow(dead_code)]

use crate::{
    symbols
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

#[derive(Clone)]
pub enum Symbol {
    Variable(symbols::Variable),
    //Function(symbols::Function)
}

type SymbolTable = Vec<Symbol>;

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    ty: FundamentalType
}

pub trait TypeCheck {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError>;
}

impl TypeCheck for NodeType {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
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
            FunctionDefinition(function_definition) => function_definition.check(symbol_table),
            Return(return_statement) => return_statement.check(symbol_table),
            VariableDeclaration(variable_declaration) => variable_declaration.check(symbol_table),
            VariableInitialisation(variable_initialisation) => variable_initialisation.check(symbol_table)
        }
    }
}

impl TypeCheck for Token {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(None)
    }
}

impl TypeCheck for expression::Boolean {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Boolean }))
    }
}

impl TypeCheck for expression::Character {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Character }))
    }
}

impl TypeCheck for expression::Float {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Float }))
    }
}

impl TypeCheck for expression::Integer {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::Integer }))
    }
}

impl TypeCheck for expression::Str {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        Ok(Some(Type{ ty: FundamentalType::String }))
    }
}

impl TypeCheck for expression::Identifier {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for expression::BinaryOperator {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        let left_table: SymbolTable = symbol_table.clone();
        let left = (*self.left)
            .check(left_table)
            .unwrap()
            .unwrap();
        let right_table: SymbolTable = symbol_table.clone();
        let right = (*self.right)
            .check(right_table)
            .unwrap()
            .unwrap();

        match self.op.value.as_str() {
            "+" | "-" => {
                let prec = vec![
                    FundamentalType::Float,
                    FundamentalType::Integer,
                    FundamentalType::Character,
                ];

                if prec.contains(&left.ty) && prec.contains(&right.ty) {
                    let left_prec = prec
                                    .iter()
                                    .position(|t| t == &left.ty);
                    let right_prec = prec
                                    .iter()
                                    .position(|t| t == &right.ty);

                    if left_prec < right_prec {
                        return Ok(Some(left));
                    }
                    else if left_prec > right_prec {
                        return Ok(Some(right));
                    }
                    else {
                        return Ok(Some(left));
                    }
                }

                if self.op.value == "+" {
                    if left.ty == FundamentalType::String || right.ty == FundamentalType::String {
                        return Ok(Some(Type{ ty: FundamentalType::String }));
                    }
                }

                //  NOT IMPLEMENTED
                Err(TypeError)
            },
            _ => {
                //  NOT IMPLEMENTED
                Err(TypeError)
            }
        }
    }
}

impl TypeCheck for node::Block {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        for stmt in &self.stmts {
            let symbol_table: SymbolTable = symbol_table.clone();
            stmt.check(symbol_table)
                .unwrap();
        }
        Ok(None)
    }
}

impl TypeCheck for node::File {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        for stmt in &self.stmts {
            let symbol_table: SymbolTable = symbol_table.clone();
            stmt.check(symbol_table)
                .unwrap();
        }
        Ok(None)
    }
}

impl TypeCheck for node::Output {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::ParameterList {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::Parameter {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::Term {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        self.node.check(symbol_table)
    }
}

impl TypeCheck for node::TypeSpecifier {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::ExpressionStatement {
    fn check(&self, symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        self.expr.check(symbol_table)
            .unwrap();
        Ok(None)
    }
}

impl TypeCheck for statement::FunctionDefinition {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::ReturnStatement {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::VariableDeclaration {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::VariableInitialisation {
    fn check(&self, _symbol_table: SymbolTable) -> Result<Option<Type>, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

pub fn annotate_file(tree: node::File) -> Result<Option<node::File>, TypeError> {
    let symbol_table: SymbolTable = vec![];
    
    tree.check(symbol_table)
        .unwrap();
    Ok(None)    // Have to work out how to go about annotating the original AST and returning that
                // instead of just checking types 
}