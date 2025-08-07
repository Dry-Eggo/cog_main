

use crate::frontend::driver::Driver;
use crate::frontend::token:: {Token, Spanned, Span};
use crate::frontend::ast:: {Item};

pub struct Parser<'a> {
    driver: &'a Driver,
    tokens: &'a [Spanned<Token<'a>>],
    pos:  usize,
    max:  usize,
}

impl<'a> Parser<'a> {
    pub fn new (driver: &'a Driver, tokens: &'a [Spanned<Token>]) -> Self  {
	Self {
	    driver,
	    tokens,
	    pos: 0,
	    max: tokens.len()
	}
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
    
    pub fn parse (&mut self) -> Spanned<Item> {
	
	
	
	Spanned::create(Item::Invalid, *self.get_span())
    }
}
