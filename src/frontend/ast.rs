#![allow(unused)]
use crate::frontend::token:: {Spanned, Span};

pub type SpannedItem<'a> = Spanned<Item<'a>>;
pub type SpannedStmt<'a> = Spanned<Stmt<'a>>;
pub type SpannedExpr<'a> = Spanned<Expr<'a>>;

#[derive(Clone)]
pub enum Item<'a> {
    FunctionDefinition ( FnDef<'a> ),
    // Place Holder for Parsing Errors
    // Won't ever reach semantics because syntax error will halt compilation
    // before semantics
    Invalid,
}

#[derive(Clone)]
pub enum Stmt<'a> {
    MutVarDecl   ( MutVarDecl<'a> ),
    CompoundStmt ( Vec<SpannedStmt<'a>> ),
    
    // An expression used solely for side-effects 
    Effector     ( Expr<'a> ),
    Invalid,
}

#[derive(Clone)]
pub enum Expr<'a> {
    Integerlit (i64),
    Stringlit  (&'a str),
    Identifier (&'a str),
    Booleanlit (bool),
    Invalid,
}

/// Function Definition
#[derive(Clone)]
pub struct FnDef<'a> {
    // points to its name in the source.
    // source lives longer than the compilation
    pub name: &'a str,    
    // span to the function name
    pub name_span: Span,
    pub body: Option<SpannedStmt<'a>>,
}


#[derive(Clone)]
pub struct MutVarDecl<'a> {
    pub name: &'a str,
    // span to the identifier in this statement
    pub name_span: Span,
    pub init: Option<SpannedExpr<'a>>,
}
