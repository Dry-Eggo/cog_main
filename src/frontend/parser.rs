#![allow(unused)]

use crate::frontend::{token::Token, span::Spanned, node::Node};

pub struct Parser<'a> {
    source:  &'a str,
    tokens:  &'a Vec<Spanned<'a, Token>>,
    program: Vec<Spanned<'a, Box<Node<'a>>>>
}

impl<'a> Parser<'a> {
    pub fn new (source: &'a str, tokens: &'a Vec<Spanned<'a, Token>>) -> Self {	
	Self {
	    source,
	    tokens,
	    program: vec![],
	}
    }

    pub fn build_ast(&mut self) -> Node<'a> {
	
	return Node::Program(self.program.clone());
    }
}
