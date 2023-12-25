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
    pub type_: Option<String>,
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

impl<'ctx> VarDeclStmt<'ctx> {
    pub fn new(name: String, type_: Option<String>, expr: Box<dyn Expr<'ctx> + 'ctx>) -> Self {
        Self {
            name,
            type_,
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

#[derive(Stmt)]
pub struct BlockStmt<'ctx> {
    pub stmts: Vec<Box<dyn Stmt<'ctx> + 'ctx>>,
}

impl<'ctx> BlockStmt<'ctx> {
    pub fn new(stmts: Vec<Box<dyn Stmt<'ctx> + 'ctx>>) -> Self {
        Self {
            stmts,
        }
    }
}

#[derive(Stmt)]
pub struct IfStmt<'ctx> {
    pub cond: Box<dyn Expr<'ctx> + 'ctx>,
    pub then: Box<dyn Stmt<'ctx> + 'ctx>,
    pub otherwise: Option<Box<dyn Stmt<'ctx> + 'ctx>>,
}

impl<'ctx> IfStmt<'ctx> {
    pub fn new(cond: Box<dyn Expr<'ctx> + 'ctx>, then: Box<dyn Stmt<'ctx> + 'ctx>, otherwise: Option<Box<dyn Stmt<'ctx> + 'ctx>>) -> Self {
        Self {
            cond,
            then,
            otherwise,
        }
    }
}

#[derive(Stmt)]
pub struct WhileStmt<'ctx> {
    pub cond: Box<dyn Expr<'ctx> + 'ctx>,
    pub body: Box<dyn Stmt<'ctx> + 'ctx>,
}

impl<'ctx> WhileStmt<'ctx> {
    pub fn new(cond: Box<dyn Expr<'ctx> + 'ctx>, body: Box<dyn Stmt<'ctx> + 'ctx>) -> Self {
        Self {
            cond,
            body,
        }
    }
}

#[derive(Stmt)]
pub struct BreakStmt;

impl BreakStmt {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Stmt)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

pub struct Param {
    pub name: String,
    pub type_: String,
}

impl Param {
    pub fn new(name: String, type_: String) -> Self {
        Self {
            name,
            type_,
        }
    }
}


#[derive(Stmt)]
pub struct FunctionDefStmt<'ctx> {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Box<dyn Stmt<'ctx> + 'ctx>,
}

impl<'ctx> FunctionDefStmt<'ctx> {
    pub fn new(name: String, params: Vec<Param>, body: Box<dyn Stmt<'ctx> + 'ctx>) -> Self {
        Self {
            name,
            params,
            body,
        }
    }
}

#[derive(Stmt)]
pub struct FunctionDeclStmt {
    pub name: String,
    pub params: Vec<Param>,
}

impl FunctionDeclStmt {
    pub fn new(name: String, params: Vec<Param>) -> Self {
        Self {
            name,
            params,
        }
    }
}
