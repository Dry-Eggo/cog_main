#![allow(unused)]

use crate::frontend::token:: {Token, Spanned, Span};
use crate::frontend::ast:: {
    SpannedItem, SpannedStmt, SpannedExpr,
    Item, Stmt, Expr,
    FnDef, RawType, MutVarDecl};
use crate::frontend::error::*;


pub struct Parser<'source> {
    tokens: &'source [Spanned<Token<'source>>],
    pos:  usize,
    max:  usize,

    errors: Diagnostics<'source>,
}

impl<'source> Parser<'source> {
    pub fn new (tokens: &'source [Spanned<Token>]) -> Self  {
	Self {
	    tokens,
	    pos: 0,
	    max: tokens.len(),
	    errors: vec![]
	}
    }

    fn matches (&self, tok: Token) -> bool {
	self.get() == tok
    }

    fn expect (&mut self, tok: Token) -> bool {
	if !self.matches(tok) {
	    return false
	}
	self.advance();
	true
    }

    fn expect_err (&mut self, tok: Token<'source>) -> bool {
	if !self.matches (tok) {
	    let span = self.get_span();
	    let got  = self.get();
	    self.errors.push(Diag::UnexpectedTokenWithEx(
		UnexpectedTokenWithExSub { expected: tok, got, span }
	    ));
	    return false
	}
	self.advance();
	true
    }

    fn expect_id (&mut self) -> &'source str {
	match self.get () {
	    Token::Identifier (s) => {
		self.advance ();
		s
	    }
	    _ => {
		let span = self.get_span ();
		self.errors.push (Diag::MissingIdentifier(span, self.before()));
		"error"
	    }
	}
    }

    fn before (&self) -> Token<'source> {	
	if self.pos <= 0 {
	    return self.tokens.last().unwrap().item
	}	
	self.tokens[self.pos - 1].item
    }
    
    fn before_span (&self) -> Span {
	if self.pos <= 0 {
	    return self.tokens.last().unwrap().span
	}
	self.tokens[self.pos - 1].span	
    }
    
    fn get (&self) -> Token<'source> {	
	if self.pos >= self.max {
	    return self.tokens.last().unwrap().item
	}	
	self.tokens[self.pos].item
    }
    
    fn get_span (&self) -> Span {
	if self.pos >= self.max {
	    return self.tokens.last().unwrap().span
	}
	self.tokens[self.pos].span	
    }

    fn advance (&mut self) {
	self.pos += 1;
    }
    
    pub fn parse (&mut self) -> Vec<SpannedItem<'source>> {	
	let mut items = vec![];	
	loop {
	    let tok = self.get();
	    if let Token::EOF = tok { break }	    
	    let item = self.parse_item ();
	    items.push (item);
	}	
	items
    }

    fn parse_item (&mut self) -> SpannedItem<'source> {
	let tok = self.get();
	match tok {
	    Token::Func => {
		self.parse_function ()
	    }
	    _   => {
		todo!("unexpected top-level item: {} {}", self.get(), self.errors.len())
	    }
	}
    }

    fn parse_function (&mut self) -> SpannedItem<'source> {	
	let start_span = self.get_span ();
	self.expect_err(Token::Func);
	
	let name_span = self.get_span ();
	let name = self.expect_id ();

	self.expect_err (Token::OParen);
	self.expect_err (Token::CParen);
	
	self.expect_err (Token::OBrace);
	let body =  Some(self.parse_body ());
	println!("{:?}", body);
	self.expect_err (Token::CBrace);
	
	let end_span = self.get_span ();
	
	Spanned::create (Item::FunctionDefinition (FnDef {
	    name,
	    name_span,
	    body,
	}), start_span.merge(&end_span))
    }

    fn parse_body (&mut self) -> SpannedStmt<'source> {
	let start_span = self.get_span ();
	let mut statements = vec![];

	while !self.matches (Token::CBrace) {
	    statements.push(self.parse_stmt ())
	}
	
	let end_span = self.get_span ();
	Spanned::create (Stmt::CompoundStmt(statements), start_span.merge(&end_span))
    }
    
    fn parse_stmt (&mut self) -> Option<SpannedStmt<'source>> {
	match self.get () {
	    Token::SemiColon => return None,
	    Token::Var       => self.parse_var_decl (),
	    _ => {
		todo!()
	    }
	}
    }

    fn parse_var_decl (&mut self) -> Option<SpannedStmt<'source>> {
	let start_span = self.get_span ();
	self.expect_err(Token::Var);
	let name_span  = self.get_span ();
	let identifier = self.expect_id ();

	if !self.expect (Token::Colon) {
	    todo!("parse_var_decl: no type is given");
	}
	let ty = self.parse_type ();
	if !self.matches (Token::SemiColon) {
	    todo!()
	}
	
	self.expect_err (Token::SemiColon);
	let end_span  = self.get_span ();
	Some (Spanned::create (Stmt::MutVarDecl(MutVarDecl{
	    name: identifier,
	    name_span,
	    init: None,
	    ty,
	}), start_span.merge(&end_span)))
    }

    fn parse_type (&mut self) -> Option<Spanned<RawType<'source>>> {
	let start_span = self.get_span ();
	match self.get() {
	    Token::PrimitiveType ( repr )  => {
		self.advance();
		let ty = match repr {
		    "i8"   => Spanned::create (RawType::SignedInteger(8), start_span),
		    "i16"  => Spanned::create (RawType::SignedInteger(16), start_span),
		    "i32"  => Spanned::create (RawType::SignedInteger(32), start_span),
		    "i64"  => Spanned::create (RawType::SignedInteger(64), start_span),
		    "cstr" => Spanned::create (RawType::CString, start_span),
		    _ => todo!()
		};
		return Some (ty)
	    }
	    _ => {
		todo!();
	    }
	}
	None
    }
}

pub fn parse_tokens<'source> (tokens: &'source [Spanned<Token<'source>>]) -> Result<Vec<SpannedItem<'source>>, Diagnostics<'source>> {
    let mut parser = Parser::new(tokens);
    let items  = parser.parse();

    if parser.errors.is_empty() {
	return Ok (items)
    }
    Err (parser.errors)
}
