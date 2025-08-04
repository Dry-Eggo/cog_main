#![allow(unused)]
use crate::frontend::{token::Token, span::{Span, Spanned}};

pub struct Lexer<'a>  {
    source: &'a str,
    filename: &'a str,
    cursor: usize,
    line  : usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
	Self {
	    source,
	    filename,
	    cursor: 0,
	    line: 1,
	    column: 1,
	}
    }

    fn now(&self) -> Option<char> {
	if self.cursor >= self.source.len() {
	    return None;
	}
	return self.source.chars().nth(self.cursor)
    }

    fn peek(&self, n: usize) -> Option<char> {
	if self.cursor + n >= self.source.len() {
	    return None;
	}
	return self.source.chars().nth(self.cursor + n)
    }

    fn advance(&mut self) -> Option<char> {
	if let Some (ch) = self.now() {
	    if ch == '\n' {
		self.line += 1;
		self.column = 1;
	    } else {
		self.column += 1;
	    }
	    self.cursor += 1;
	    return Some(ch);
	}
	return None;
    }
    
    pub fn parse_name(&mut self, tokens: &mut Vec<Spanned<'a, Token>>) {
	let mut buffer = String::new();
	let sl = self.line;
	let sc = self.column;
	
	while self.now().is_some() && (self.now().unwrap().is_alphanumeric() || self.now().unwrap() == '_') {
	    buffer.push(self.advance().unwrap());
	}
	
	let token = match buffer.as_str() {
	    "func" => Token::Func,
	    "let"  => Token::Let,
	    _      => Token::Identifier(buffer)
	};
	
	tokens.push(Spanned::span(token, Span::new(self.filename, sl, sc, self.column-1)));
    }

    pub fn parse_digit(&mut self, tokens: &mut Vec<Spanned<'a, Token>>) {
	let mut buffer = String::new();
	let sl = self.line;
	let sc = self.column;	

	while self.now().is_some() && self.now().unwrap().is_numeric() {
	    buffer.push(self.advance().unwrap());
	}
	
	let token = Token::Integer(buffer);
	tokens.push(Spanned::span(token, Span::new(self.filename, sl, sc, self.column-1)));	
    }
    
    pub fn lex(&mut self) -> Vec<Spanned<Token>> {
	let mut tokens = vec![];

	while self.now().is_some() {

	    if self.now().unwrap().is_whitespace() {
		self.advance();
		continue;
	    }
	    
	    if self.now().unwrap().is_alphabetic() {
		self.parse_name(&mut tokens);
		continue;
	    }

	    if self.now().unwrap().is_numeric() {
		self.parse_digit(&mut tokens);
		continue;
	    }
	    
	    let sl = self.line;
	    let sc = self.column;
	    match self.now().unwrap() {
		'=' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::Eq, Span::new(self.filename, sl, sc, self.column-1)));
		}
		';' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::SemiColon, Span::new(self.filename, sl, sc, self.column-1)));
		}
		'{' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::OBrace, Span::new(self.filename, sl, sc, self.column-1)));
		}
		'}' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::CBrace, Span::new(self.filename, sl, sc, self.column-1)));
		}
		'(' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::OParen, Span::new(self.filename, sl, sc, self.column-1)));
		}
		')' => {
		    self.advance();
		    tokens.push(Spanned::span(Token::CParen, Span::new(self.filename, sl, sc, self.column-1)));
		}
		':' => {
		    self.advance();

		    if let Some('=') = self.now() {
			self.advance();
			tokens.push(Spanned::span(Token::Coleq, Span::new(self.filename, sl, sc, self.column-1)));
			continue;
		    }
		    
		    tokens.push(Spanned::span(Token::Colon, Span::new(self.filename, sl, sc, self.column-1)));
		}
		_ => {
		    println!("Unknown character '{}'", self.now().unwrap());
		    self.advance();
		}
	    }
	}
	
	tokens.push(Spanned::span(Token::EOF, Span::new(self.filename, self.line, self.column, self.column)));	
	return tokens;
    }
}
