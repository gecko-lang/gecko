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

#[enum_dispatch(NodeType)]
pub trait TypeCheck {
    fn check(&self) -> Result<Type, TypeError>;
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

pub fn annotate_file(tree: node::File) -> Result<Option<node::File>, TypeError> {
    Ok(None)
}