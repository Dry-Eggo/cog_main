#![allow(unused)]

use crate::frontend:: {span::Spanned, span::Span};

pub type SpannedExpr<'a> = Spanned<'a, Box<Expr<'a>>>;
pub type SpannedStmt<'a> = Spanned<'a, Box<Stmt<'a>>>;
pub type SpannedItem<'a> = Spanned<'a, Box<Item<'a>>>;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    NoOp,
    Expr (SpannedExpr<'a>),
    Stmt (SpannedStmt<'a>),
    Item (SpannedItem<'a>),
    
    Program (Vec<Spanned<'a, Box<Item<'a>>>>)
}
#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub name:  String,
    params:    Option<Vec<String>>, // TODO
    body:      Option<SpannedStmt<'a>>
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    NoOp,
    Function (Function<'a>)
}

impl<'a> Item<'a> {
    pub fn make_function(name: String, params: Option<Vec<String>>, body: Option<SpannedStmt<'a>>) -> Self {
	Item::Function ( Function {
	    name,
	    params,
	    body,
	})
    }
}
#[derive(Debug, Clone)]
pub struct LetBinding<'a> {
    name:     Spanned<'a, String>,
    rhs:      SpannedExpr<'a>,
    modifier: BindingModifier
}

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    NoOp,
    Expr (SpannedExpr<'a>),
    LetBinding (LetBinding<'a>),
    CompoundStmt (Vec<SpannedStmt<'a>>)
}

impl<'a> Stmt<'a> {
    pub fn make_binding(name: Spanned<'a, String>, bmod: BindingModifier, rhs: SpannedExpr<'a>, span: Span<'a>) -> SpannedStmt<'a> {
	return Spanned::span(Box::new(Stmt::LetBinding (LetBinding {
		name,
		modifier: bmod,
		rhs,
	})), span);
    }

    pub fn make_no_op(span: Span<'a>) -> SpannedStmt<'a> {
	return Spanned::span(Box::new(Stmt::NoOp), span);
    }
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
	lhs:  SpannedExpr<'a>,
	rhs:  SpannedExpr<'a>,
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div
}

impl<'a> Expr<'a> {
    pub fn make_integer(value: i64, span: Span<'a>) -> SpannedExpr<'a> {
	return Spanned::span(Box::new(Expr::Integer(value)), span);
    }

    pub fn make_no_op(span: Span<'a>) -> SpannedExpr<'a> {
	return Spanned::span(Box::new(Expr::NoOp), span);
    }
}
