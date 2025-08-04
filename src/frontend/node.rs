#![allow(unused)]

use crate::frontend:: {span::Spanned};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    NoOp,
    Expr (Spanned<'a, Box<Expr<'a>>>),
    Stmt (Spanned<'a, Box<Stmt<'a>>>),
    Item (Spanned<'a, Box<Item<'a>>>),
    
    Program (Vec<Spanned<'a, Box<Item<'a>>>>)
}
#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub name:    String,
    params:  Option<Vec<String>>, // TODO
    body:    Option<Spanned<'a, Box<Stmt<'a>>>>
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    NoOp,
    Function (Function<'a>)
}

impl<'a> Item<'a> {
    pub fn make_function(name: String, params: Option<Vec<String>>, body: Option<Spanned<'a, Box<Stmt<'a>>>>) -> Self {
	Item::Function ( Function {
	    name,
	    params,
	    body,
	})
    }
}

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    NoOp,
    Expr (Spanned<'a, Box<Expr<'a>>>),
    LetBinding {
	name:    Spanned<'a, String>,
	rhs:     Spanned<'a, Box<Expr<'a>>>,
	modfier: Spanned<'a, BindingModifier>
    },
    CompoundStmt (Vec<Spanned<'a, Box<Stmt<'a>>>>)
}

#[derive(Debug, Clone)]
pub enum BindingModifier {
    Constant,
    Mutable,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    NoOp,
    Integer(i64),
    StringExpr(String),
    Identifier(String),

    BinaryExpr {
	op:   BinaryOp,
	lhs:  Spanned<'a, Box<Expr<'a>>>,
	rhs:  Spanned<'a, Box<Expr<'a>>>,
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div
}
