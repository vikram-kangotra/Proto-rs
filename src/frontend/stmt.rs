use super::{visitor::StmtVisitor, expr::Expr, type_::Type};

use proto_rs_macros::Stmt;

pub trait Stmt<'ctx> {
    fn accept(&self, visitor: &mut dyn StmtVisitor<'ctx>);
}

#[derive(Stmt)]
pub struct ExprStmt<'ctx> {
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Stmt)]
pub struct VarDeclStmt<'ctx> {
    pub name: String,
    pub type_: Type,
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Stmt)]
pub struct ReturnStmt<'ctx> {
    pub expr: Box<dyn Expr<'ctx> + 'ctx>,
}

#[derive(Stmt)]
pub struct BlockStmt<'ctx> {
    pub stmts: Vec<Box<dyn Stmt<'ctx> + 'ctx>>,
}

#[derive(Stmt)]
pub struct IfStmt<'ctx> {
    pub cond: Box<dyn Expr<'ctx> + 'ctx>,
    pub then: Box<dyn Stmt<'ctx> + 'ctx>,
    pub otherwise: Option<Box<dyn Stmt<'ctx> + 'ctx>>,
}

#[derive(Stmt)]
pub struct WhileStmt<'ctx> {
    pub cond: Box<dyn Expr<'ctx> + 'ctx>,
    pub body: Box<dyn Stmt<'ctx> + 'ctx>,
}

#[derive(Stmt)]
pub struct BreakStmt;

#[derive(Stmt)]
pub struct ContinueStmt;


#[derive(Clone, Eq, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_: Type,
}

impl Param {
    pub fn new(name: String, type_: Type) -> Self {
        Self {
            name,
            type_,
        }
    }
}

#[derive(Stmt)]
pub struct FunctionDeclStmt {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
}

#[derive(Stmt)]
pub struct FunctionDefStmt<'ctx> {
    pub func_decl: FunctionDeclStmt,
    pub body: Box<dyn Stmt<'ctx> + 'ctx>,
}
