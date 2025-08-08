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

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    MutVarDecl   ( MutVarDecl<'a> ),
    CompoundStmt ( Vec<Option<SpannedStmt<'a>>> ),
    
    // An expression used solely for side-effects 
    Effector     ( Expr<'a> ),
    Invalid,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Integerlit (i64),
    Stringlit  (&'a str),
    Identifier (&'a str),
    Booleanlit (bool),
    Invalid,
}

/// Function Definition
#[derive(Debug, Clone)]
pub struct FnDef<'a> {
    // points to its name in the source.
    // source lives longer than the compilation
    pub name: &'a str,    
    // span to the function name
    pub name_span: Span,
    pub body: Option<SpannedStmt<'a>>,
}

#[derive(Debug, Clone)]
pub struct MutVarDecl<'a> {
    pub name: &'a str,
    // span to the identifier in this statement
    pub name_span: Span,
    pub init: Option<SpannedExpr<'a>>,
    pub ty:   Option<Spanned<RawType<'a>>>,
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub enum RawType<'a> {
    // i8-64
    SignedInteger (/* bits:*/ u8),
    // u8-64
    UnsignedInteger (/* bits:*/ u8),
    String {
	// Cog string are immutable an thier length needs to be known at compile-time
	// if len is `None`, then it will be inferred during semantics
	len: Option<usize>,
    },
    // Cog's primitive type for Dynamically growing strings
    DString,
    // FFI-Compatible, Null terminated Strings,
    CString,
    // A type to represent a single byte of memory.
    // usually 'u8'
    Byte,
    Bool,    
    // void
    None,
    // *T
    Pointer (Box<RawType<'a>>),
    // &mut T / &T
    // Still a pointer, but like Rust, references support 'auto deref'
    Reference {
	to: Box<RawType<'a>>,
	mutablity: Mutability,
    },
    // for Custom named types
    Named (&'a str),
}
