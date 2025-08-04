#![allow(unused)]

use crate::frontend:: {token::Token, span:: {Spanned, Span}};
use crate::frontend::diagnostic:: {SyntaxError, DiagnosticEngine};
use crate::frontend::node:: {Node, Stmt, Expr, Item};

pub struct Parser<'a> {
    source:      &'a str,
    tokens:      &'a [Spanned<'a, Token>],
    program:     Vec<Spanned<'a, Box<Item<'a>>>>,
    cursor:      usize,

    diagnostics: DiagnosticEngine<'a>,
}

impl<'a> Parser<'a> {
    pub fn new (source: &'a str, tokens: &'a [Spanned<'a, Token>]) -> Self {	
	Self {
	    source,
	    tokens,
	    program: vec![],
	    cursor: 0,

	    diagnostics: DiagnosticEngine::new()
	}
    }

    fn add_error(&mut self, message: String, span: Span<'a>) {
	self.diagnostics.add_syntax_error(SyntaxError::new(message, span));
    }

    fn matches(&self, token: &Token) -> bool {
	if self.next_token().item == *token {
	    return true;
	}
	return false;
    }

    fn recover(&mut self, stop: &Token) {
	while !self.matches(stop) {
	    self.advance();
	}
    }
    
    fn expect(&mut self, token: &Token) -> bool {
	if self.matches(token) {
	    self.advance();
	    return true;
	}
	
	return false;
    }   
    
    fn expect_err(&mut self, token: &Token) {
	if !self.expect(token) {
	    let tok = self.next_token();
	    self.add_error(format!("Expected {:?} but got {:?} instead", token, tok), tok.span.clone());
	}
    }

    fn expect_ident(&mut self) -> Option<String> {
	let tok = &self.next_token().item;
	if let Token::Identifier(ident) = tok {
	    // not allowed by rust
	    /* self.advance(); */
	    return Some(ident.to_string());
	}
	None
    }
    
    fn next_token(&self) -> &Spanned<'a, Token> {
	if self.cursor >= self.tokens.len() {
	    return self.tokens.last().unwrap();
	}
	
	return self.tokens.get(self.cursor).unwrap();	
    }    

    // Returns a Copy of the span held by the current token
    fn get_span(&self) -> Span<'a> {
	return self.next_token().span.clone();
    }

    fn advance(&mut self) {
	self.cursor += 1;
    }
    
    pub fn build_ast(&mut self) -> Node<'a> {

	while self.next_token().item != Token::EOF {
	    self.parse_item();
	}
	
	return Node::Program(self.program.clone());
    }

    fn parse_item(&mut self) {
	if self.matches(&Token::Func) {
	    let func = self.parse_function();
	    self.program.push(func);
	}
    }

    fn parse_function(&mut self) -> Spanned<'a, Box<Item<'a>>> {
	let start = self.get_span();
	self.advance(); // the 'func' keyword

	let function_name = {
	    if let Some(name) = self.expect_ident() {
		self.advance();
		name
	    } else {
		let end   = self.get_span();
		self.add_error("Expected an identifier after 'func'".to_string(), self.get_span());
		
		self.recover(&Token::OBrace);
		
		"error".to_string()
	    }
	};
	
	self.expect_err(&Token::OParen);
	// TODO: parse parameters
	self.expect_err(&Token::CParen);

	let mut function_body = None;
	if self.matches(&Token::OBrace) {
	    function_body = Some(self.parse_body());
	    self.expect_err(&Token::CBrace);
	}
	
	let end   = self.get_span();	
	return Spanned::span(Box::new(Item::make_function(function_name, None, function_body)), Span::merge(start, end));
    }

    fn parse_body(&mut self) -> Spanned<'a, Box<Stmt<'a>>> {
	let start = self.get_span();
	let statements = vec![];
	self.advance(); // skip '{'
	
	while !self.matches(&Token::EOF) && !self.matches(&Token::CBrace) {
	    let statement = self.parse_statement();
	}
	
	let end = self.get_span();
	return Spanned::span(Box::new(Stmt::CompoundStmt(statements)), Span::merge(start, end));
    }

    fn parse_statement(&mut self) -> Spanned<'a, Box<Stmt<'a>>> {
	let start = self.get_span();

	if self.matches(&Token::Let) {
	    todo!();
	} else {
	    let end = self.get_span();	    
	    let expr = self.parse_expr();

	    return Spanned::span(Box::new(Stmt::Expr(expr)), Span::merge(start, end));
	}	
    }

    fn parse_expr(&mut self) -> Spanned<'a, Box<Expr<'a>>> {
	println!("{:?}", self.next_token());
	todo!("parse_expr");
    }
}
