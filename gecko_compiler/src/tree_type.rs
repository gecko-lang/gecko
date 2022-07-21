//#[allow(dead_code)]

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

#[derive(Debug, PartialEq)]
pub enum FundamentalType {
    Boolean,
    Character,
    Integer,
    Float,
    String,
    DefinedType
}

#[derive(Debug, PartialEq)]
pub struct Type {
    ty: FundamentalType
}

pub trait TypeCheck {
    fn check(&self) -> Result<Type, TypeError>;
}

impl TypeCheck for NodeType {
    fn check(&self) -> Result<Type, TypeError> {
        use NodeType::*;
        match self {
            Token(token) => token.check(),

            BinaryOperator(binary_operator) => binary_operator.check(),
            Boolean(boolean) => boolean.check(),
            Character(character) => character.check(),
            Float(float) => float.check(),
            Integer(integer) => integer.check(),
            Identifier(identifier) => identifier.check(),
            Str(string) => string.check(),

            Block(block) => block.check(),
            File(file) => file.check(),
            Output(output) => output.check(),
            ParameterList(parameter_list) => parameter_list.check(),
            Parameter(parameter) => parameter.check(),
            Term(term) => term.check(),
            TypeSpecifier(type_specifier) => type_specifier.check(),

            Expression(expression) => expression.check(),
            FunctionDefinition(function_definition) => function_definition.check(),
            Return(return_statement) => return_statement.check(),
            VariableDeclaration(variable_declaration) => variable_declaration.check(),
            VariableInitialisation(variable_initialisation) => variable_initialisation.check()
        }
    }
}

impl TypeCheck for Token {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for expression::Boolean {
    fn check(&self) -> Result<Type, TypeError> {
        Ok(Type{ ty: FundamentalType::Boolean })
    }
}

impl TypeCheck for expression::Character {
    fn check(&self) -> Result<Type, TypeError> {
        Ok(Type{ ty: FundamentalType::Character })
    }
}

impl TypeCheck for expression::Float {
    fn check(&self) -> Result<Type, TypeError> {
        Ok(Type{ ty: FundamentalType::Float })
    }
}

impl TypeCheck for expression::Integer {
    fn check(&self) -> Result<Type, TypeError> {
        Ok(Type{ ty: FundamentalType::Integer })
    }
}

impl TypeCheck for expression::Str {
    fn check(&self) -> Result<Type, TypeError> {
        Ok(Type{ ty: FundamentalType::String })
    }
}

impl TypeCheck for expression::Identifier {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for expression::BinaryOperator {
    fn check(&self) -> Result<Type, TypeError> {
        let left = (*self.left).check().unwrap();
        let right = (*self.right).check().unwrap();

        match self.op.value.as_str() {
            "+" | "-" => {
                let prec = vec![
                    FundamentalType::Float,
                    FundamentalType::Integer,
                    FundamentalType::Character,
                ];

                if prec.contains(&left.ty) && prec.contains(&right.ty) {
                    let left_prec = prec.iter().position(|t| t == &left.ty);
                    let right_prec = prec.iter().position(|t| t == &right.ty);

                    if left_prec < right_prec {
                        return Ok(left);
                    }
                    else if left_prec > right_prec {
                        return Ok(right);
                    }
                    else {
                        return Ok(left);
                    }
                }

                if self.op.value == "+" {
                    if left.ty == FundamentalType::String || right.ty == FundamentalType::String {
                        return Ok(Type{ ty: FundamentalType::String });
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
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::File {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::Output {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::ParameterList {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::Parameter {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::Term {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for node::TypeSpecifier {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::ExpressionStatement {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::FunctionDefinition {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::ReturnStatement {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::VariableDeclaration {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

impl TypeCheck for statement::VariableInitialisation {
    fn check(&self) -> Result<Type, TypeError> {
        //  NOT IMPLEMENTED
        Err(TypeError)
    }
}

pub fn annotate_file(_tree: node::File) -> Result<Option<node::File>, TypeError> {
    Ok(None)
}