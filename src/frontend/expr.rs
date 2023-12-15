use inkwell::context::{Context, self};
use inkwell::types::{IntType, FloatType, StringRadix};
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

    pub fn new_bool(context: &'ctx Context, value: bool) -> Self {
        Self {
            value: context.bool_type().const_int(value as u64, false).into(),
        } 
    }

    pub fn new_char(context: &'ctx Context, value: char) -> Self {
        Self {
            value: context.i32_type().const_int(value as u64, false).into(),
        } 
    }

    pub fn new_int(context: &'ctx Context, value: i128) -> Self {
        let type_ = Self::minimize_int_type(context, value);
        Self {
            value: type_.const_int(value as u64, false).into(),
        }
    }

    pub fn new_float(context: &'ctx Context, value: f64) -> Self {
        let type_ = Self::minimize_float_type(context, value);
        Self {
            value: type_.const_float(value).into(),
        } 
    }

    fn minimize_int_type(context: &'ctx Context, value: i128) -> IntType {
        match value {
            value if value >= i8::MIN as i128 && value <= i8::MAX as i128 => context.i8_type(),
            value if value >= i16::MIN as i128 && value <= i16::MAX as i128 => context.i16_type(),
            value if value >= i32::MIN as i128 && value <= i32::MAX as i128 => context.i32_type(),
            value if value >= i64::MIN as i128 && value <= i64::MAX as i128 => context.i64_type(),
            _ => context.i128_type(),
        }
    }

    fn minimize_float_type(context: &'ctx Context, value: f64) -> FloatType {
        match value {
            value if value >= f32::MIN as f64 && value <= f32::MAX as f64 => context.f32_type(),
            value if value >= f64::MIN as f64 && value <= f64::MAX as f64 => context.f64_type(),
            _ => context.f128_type(),
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
