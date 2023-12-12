use inkwell::context::Context;
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

#[derive(Expr)]
pub struct LiteralExpr<'ctx> {
    pub value: BasicValueEnum<'ctx>,
}

impl<'ctx> LiteralExpr<'ctx> {

    pub fn new_char(context: &'ctx Context, value: char) -> Self {
        Self {
            value: context.i32_type().const_int(value as u64, false).into(),
        } 
    }

    pub fn new_int(context: &'ctx Context, value: i128) -> Self {
        Self {
            value: context.i128_type().const_int(value as u64, false).into(),
        } 
    }

    pub fn new_int8(context: &'ctx Context, value: i8) -> Self {
        Self {
            value: context.i8_type().const_int(value as u64, false).into(),
        } 
    }

    pub fn new_float(context: &'ctx Context, value: f64) -> Self {
        Self {
            value: context.f128_type().const_float(value).into(),
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
