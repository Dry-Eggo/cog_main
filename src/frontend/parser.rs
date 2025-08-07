

use crate::frontend::driver::Driver;
use crate::frontend::token:: {Token, Spanned, Span};
use crate::frontend::ast:: {SpannedItem};
use crate::frontend::error::SyntaxError;

pub struct Parser<'a> {
    driver: &'a Driver,
    tokens: &'a [Spanned<Token<'a>>],
    pos:  usize,
    max:  usize,

    errors: Vec<SyntaxError>,
}

impl<'a> Parser<'a> {
    pub fn new (driver: &'a Driver, tokens: &'a [Spanned<Token>]) -> Self  {
	Self {
	    driver,
	    tokens,
	    pos: 0,
	    max: tokens.len(),
	    errors: vec![]
	}
    }


    fn matches (&self, tok: &Token) -> bool {
	self.get() == tok
    }

    fn expect (&mut self, tok: &Token) -> bool {
	if !self.matches(tok) {
	    return false
	}

	self.advance();
	true
    }

    fn expect_err (&mut self, tok: &Token) -> bool {
	if !self.expect (tok) {
	    let span = self.get_span();
	    let got  = self.get();
	    self.errors.push(SyntaxError::new (format!("expected '{}' but got '{}' instead", tok, got), *span));
	    return false
	}
	true
    }

    fn expect_id (&mut self) -> &'a str {
	let tok = self.get();
	
	if let Token::Identifier (s) = tok {
	    return s
	}
	
	let span = self.get_span();
	self.errors.push (SyntaxError::new (format!("expected an identifer"), *span));
	"error"
    }
    
    fn get (&self) -> &Token {
	
	if self.pos >= self.max {
	    return &self.tokens.last().unwrap().item
	}
	
	&self.tokens[self.pos].item
    }
    
    fn get_span (&self) -> &Span {
	if self.pos >= self.max {
	    return &self.tokens.last().unwrap().span
	}
	&self.tokens[self.pos].span	
    }

    fn advance (&mut self) {
	self.pos += 1;
    }
    
    pub fn parse (&mut self) -> Vec<SpannedItem<'a>> {	

	let items = vec![];
	
	loop {

	    let tok = self.get();
	    if let Token::EOF = tok {
		break
	    }

	    self.parse_item ();
	    
	}
	
	items
    }

    fn parse_item (&mut self) -> SpannedItem<'a> {
	let tok = self.get();
	match tok {
	    &Token::Func => {
		self.parse_function ()
	    }
	    _   => {
		todo!("unexpected top-level item")
	    }
	}
    }

    fn parse_function (&mut self) -> SpannedItem<'a> {
	self.expect_err(&Token::Func);
	let name = self.expect_id ();
	println!("{name}");
	todo!()
    }
}
