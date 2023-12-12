use super::{visitor::Visitor, expr::Expr};

use proto_rs_macros::Stmt;

pub trait Stmt<'ctx> {
    fn accept(&self, visitor: &mut dyn Visitor<'ctx>);
}

#[derive(Stmt)]
pub struct ExprStmt<'ctx> {
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> ExprStmt<'ctx> {
    pub fn new(expr: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            expr,
        }
    }
}

#[derive(Stmt)]
pub struct VarDeclStmt<'ctx> {
    pub name: String,
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> VarDeclStmt<'ctx> {
    pub fn new(name: String, expr: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            name,
            expr,
        }
    }
}

#[derive(Stmt)]
pub struct ReturnStmt<'ctx> {
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> ReturnStmt<'ctx> {
    pub fn new(expr: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            expr,
        }
    }
}
