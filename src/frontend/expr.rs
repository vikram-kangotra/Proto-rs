use crate::frontend::visitor::ExprVisitor;
use crate::frontend::token::Token;

use proto_rs_macros::Expr;

use super::value::{LiteralValue, Value};

pub trait Expr<'ctx> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<'ctx>) -> Value<'ctx>;
}

#[derive(Expr)]
pub struct BinaryExpr<'ctx> {
    pub left: Box<dyn Expr<'ctx> + 'ctx>,
    pub op: Token,
    pub right: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Expr)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

#[derive(Expr)]
pub struct UnaryExpr<'ctx> {
    pub op: Token,
    pub right: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Expr)]
pub struct VariableExpr {
    pub name: String,
}

#[derive(Expr)]
pub struct VarAssignExpr<'ctx> {
    pub name: String,
    pub value: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Expr)]
pub struct CallExpr<'ctx> {
    pub callee: String,
    pub args: Vec<Box<dyn Expr<'ctx> + 'ctx>>,
}

#[derive(Expr)]
pub struct ListExpr<'ctx> {
    pub values: Vec<Box<dyn Expr<'ctx> + 'ctx>>,
}

#[derive(Expr)]
pub struct IndexExpr<'ctx> {
    pub variable: VariableExpr,
    pub indices: Vec<Box<dyn Expr<'ctx> + 'ctx>>,
}
