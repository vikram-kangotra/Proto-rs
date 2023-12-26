use inkwell::values::BasicValueEnum;
use crate::frontend::visitor::Visitor;
use crate::frontend::token::Token;

use proto_rs_macros::Expr;

pub trait Expr<'ctx> {
    fn accept(&self, visitor: &mut dyn Visitor<'ctx>) -> BasicValueEnum<'ctx>;
}

#[derive(Expr)]
pub struct BinaryExpr<'ctx> {
    pub left: Box<dyn Expr<'ctx> + 'ctx>,
    pub op: Token,
    pub right: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> BinaryExpr<'ctx> {
    pub fn new(left: Box<dyn Expr<'ctx> + 'ctx>, op: Token, right: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            left,
            op,
            right,
        }
    }
}

#[derive(Copy, Clone)]
pub enum IntType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Copy, Clone)]
pub enum FloatType {
    F32(f32),
    F64(f64),
}

#[derive(Copy, Clone)]
pub enum LiteralType {
    Bool(bool),
    Char(char),
    Int(IntType),
    Float(FloatType),
}

#[derive(Expr)]
pub struct LiteralExpr {
    pub value: LiteralType,
}

impl LiteralExpr {

    pub fn new(value: LiteralType) -> Self {
        Self {
            value,
        }
    }
}

#[derive(Expr)]
pub struct UnaryExpr<'ctx> {
    pub op: Token,
    pub right: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> UnaryExpr<'ctx> {
    pub fn new(op: Token, right: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            op,
            right,
        }
    }
}

#[derive(Expr)]
pub struct VariableExpr {
    pub name: String,
}

impl<'ctx> VariableExpr {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

#[derive(Expr)]
pub struct VarAssignExpr<'ctx> {
    pub name: String,
    pub value: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> VarAssignExpr<'ctx> {
    pub fn new(name: String, value: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            name,
            value,
        }
    }
}

#[derive(Expr)]
pub struct CallExpr<'ctx> {
    pub callee: String,
    pub args: Vec<Box<dyn Expr<'ctx> + 'ctx>>,
}

impl<'ctx> CallExpr<'ctx> {
    pub fn new(callee: String, args: Vec<Box<dyn Expr<'ctx> + 'ctx>>) -> Self {
        Self {
            callee,
            args,
        }
    }
}
