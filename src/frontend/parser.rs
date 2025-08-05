#![allow(unused)]
use crate::frontend::arena::Allocator;
use crate::frontend:: {token::Token, span:: {Spanned, Span}};
use crate::frontend::diagnostic:: {SyntaxError, DiagnosticEngine};
use crate::frontend::node:: {Node, Stmt, Expr, Item, self};
use crate::frontend::node:: {SpannedStmt, SpannedExpr, SpannedItem};

pub struct Parser<'a> {
    tokens:      &'a [Spanned<'a, Token>],
    program:     Vec<Spanned<'a, Box<Item<'a>>>>,
    cursor:      usize,

    diagnostics: DiagnosticEngine<'a>,
}

impl<'a> Parser<'a> {
    pub fn new (tokens: &'a [Spanned<'a, Token>], arena: *mut Allocator) -> *mut Self {
	unsafe {
	    let parser = (*arena).alloc_ty::<Parser>();
	    (*parser).tokens      = tokens;
	    (*parser).program     = vec![];
	    (*parser).cursor      = 0;
	    (*parser).diagnostics = DiagnosticEngine::new();
	    parser
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
	let is_ident = matches!(self.next_token().item, Token::Identifier(_));

	if is_ident {
	    if let Token::Identifier(ref ident) = self.next_token().item {
		return Some(ident.to_string());
	    }
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
	let mut statements = vec![];
	self.advance(); // skip '{'
	
	while !self.matches(&Token::EOF) && !self.matches(&Token::CBrace) {
	    let statement = self.parse_statement();

	    if self.matches(&Token::SemiColon) {
		self.advance();
	    }
	    
	    statements.push(statement);
	}
	
	let end = self.get_span();
	return Spanned::span(Box::new(Stmt::CompoundStmt(statements)), Span::merge(start, end));
    }

    fn parse_statement(&mut self) -> Spanned<'a, Box<Stmt<'a>>> {
	let start = self.get_span();

	if self.matches(&Token::Let) {
	    let let_stmt = self.parse_binding();
	    return let_stmt;
	} else {
	    let expr = self.parse_expr();
	    let end = self.get_span();	    

	    return Spanned::span(Box::new(Stmt::Expr(expr)), Span::merge(start, end));
	}	
    }

    fn parse_binding(&mut self) -> SpannedStmt<'a> {
	let start = self.get_span();
	self.advance();	
	let mut bmod = node::BindingModifier::Constant;

	let var_name = {
	    let span = self.get_span();
	    if let Some(name) = self.expect_ident() {
		self.advance();
		Spanned::span(name, span)
	    } else {	    
		self.add_error("Expected a variable name".to_string(), self.get_span());	    
		self.recover(&Token::SemiColon);	    
		return Stmt::make_no_op(self.get_span());
	    }
	    
	};

	self.expect_err(&Token::Eq);
	let rhs = self.parse_expr();
	
	let end = self.get_span();
	return Stmt::make_binding(var_name, bmod, rhs, Span::merge(start, end));
    }
    
    fn parse_expr(&mut self) -> SpannedExpr<'a> {
	return self.parse_logical_or();
    }

    fn parse_logical_or(&mut self) -> SpannedExpr<'a> {
	return self.parse_logical_and();
    }

    fn parse_logical_and(&mut self) -> SpannedExpr<'a> {
	return self.parse_equality();
    }

    fn parse_equality(&mut self) -> SpannedExpr<'a> {
	return self.parse_additive();
    }

    fn parse_additive(&mut self) -> SpannedExpr<'a> {
	return self.parse_term();
    }
    
    fn parse_term(&mut self) -> SpannedExpr<'a> {
	return self.parse_postfix();
    }

    fn parse_postfix(&mut self) -> SpannedExpr<'a> {
	return self.parse_atom();
    }
    
    fn parse_atom(&mut self) -> SpannedExpr<'a> {	
	let span = self.get_span();
	match self.next_token().item {
	    Token::Integer(ref num) => {
		let value = num.parse::<i64>().unwrap();
		self.advance();
		return Expr::make_integer(value, span.clone());
	    }
	    _ => {
		self.add_error("Not a valid expression".to_string(), span.clone());
		self.recover(&Token::SemiColon);
		return Expr::make_no_op(span.clone());
	    }
	}
    }    
}
