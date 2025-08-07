
use crate::frontend::driver::Driver;
use crate::frontend::token:: {Token, Spanned};

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    driver: &'a Driver,
    cursor: usize,
    line:   usize,
    col:    usize
}

impl<'a> Lexer<'a> {
    pub fn new (driver: &'a Driver) -> Self {
	Self {
	    driver,
	    cursor: 0,	    
	    line: 1,
	    col: 1,
	}
    }


    fn peek (&mut self, n: usize) -> Option<char> {
	self.driver.source[self.cursor..].chars().nth(n)
    }

    fn advance(&mut self) -> Option<char> {
	if let Some( ch ) = self.peek(0) {
	    if ch == '\n' {
		self.line += 1;
		self.col  = 1;
	    } else {
		self.col += 1;		
	    }
	    self.cursor += ch.len_utf8();
	    return Some (ch)
	}
	None
    }

    pub fn parse_name (&mut self) -> Spanned<Token<'a>> {
	let sl = self.line;
	let sc = self.cursor;

	while let Some ( ch ) = self.peek(0) {
	    if ch.is_ascii_alphanumeric() || ch == '_' {
		self.advance();
	    } else {
		break;
	    }
	}
	let slice = &self.driver.source[sc..self.col-1];
	match slice {
	    "fn"   => Spanned::wrap(Token::Func, sl, sc, self.col-1),
	    "let"  => Spanned::wrap(Token::Let,  sl, sc, self.col-1),
	    "var"  => Spanned::wrap(Token::Var,  sl, sc, self.col-1),

	    _      => Spanned::wrap(Token::Identifier(slice), sl, sc, self.col-1),
	}
    }
    
    pub fn lex (&mut self) -> Vec<Spanned<Token<'a>>> {
	let mut tokens  = vec![];
	while let Some (ch) = self.peek(0) {

	    if ch.is_whitespace() {
		self.advance();
		continue;
	    }

	    if ch.is_alphabetic() || ch == '_' {
		tokens.push(self.parse_name());
		continue;
	    }
	    
	    let sl = self.line;
	    let sc = self.cursor;	    
	    match ch {
		'{' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::OBrace, sl, sc, self.col));
		}
		'}' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::CBrace, sl, sc, self.col));
		}
		'(' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::OParen, sl, sc, self.col));
		}
		')' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::CParen, sl, sc, self.col));
		}
		_ => {
		    eprintln!("Unexpected character found in file: {}:{}: '{}'", sl, sc, ch);
		    self.advance();
		}		
	    }
	}
	
	tokens.push (Spanned::wrap (Token::EOF, 0, 0, 0));
	tokens
    }
}
