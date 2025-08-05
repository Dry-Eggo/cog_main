#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::frontend::token::*;
use crate::dref;
use crate::utils::string::*;
use crate::utils::array::*;

pub type SpannedStmt = Spanned<*mut Stmt>;
pub type SpannedItem = Spanned<*mut Item>;
pub type SpannedExpr = Spanned<*mut Expr>;

pub enum Item {
    FunctionDef(*mut FunctionDef),
}

pub enum Stmt {
    Empty,
    Expr (SpannedExpr),
    CompoundStmt(*mut CogArray<SpannedStmt>)
}

pub enum Expr {
    Integer(i64),
    String (CogString),
    Identifier (CogString),
}

pub struct FunctionDef {
    name: CogString,
    body: *mut Stmt,
}
