
use crate::frontend::token:: {Token, Spanned, Span};
use crate::frontend::driver:: {SourceMap, SourceFile};
use crate::frontend::error::*;

#[derive(Clone, Debug)]
pub struct Lexer<'source> {
    source_map: &'source SourceMap,
    source_id: SourceFile,
    source:      &'source String,
    cursor:      usize,
    line:        usize,
    col:         usize,
    diagnostics: Diagnostics<'source>,
}

impl<'source> Lexer<'source> {
    pub fn new (source_id: SourceFile, smap: &'source SourceMap) -> Self {
	let source = smap.get_source_by_id (source_id);
	Self {
	    source_id,
	    source_map: smap,
	    source,
	    cursor: 0,	    
	    line: 1,
	    col: 1,
	    diagnostics: vec![],
	}
    }


    fn peek (&mut self, n: usize) -> Option<char> {
	self.source[self.cursor..].chars().nth(n)
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

    pub fn parse_name (&mut self) -> Spanned<Token<'source>> {
	let sl = self.line;
	let sc = self.cursor;

	while let Some ( ch ) = self.peek(0) {
	    if ch.is_ascii_alphanumeric() || ch == '_' {
		self.advance();
	    } else {
		break;
	    }
	}
	let slice = &self.source[sc..self.col-1];
	match slice {
	    "fn"   => Spanned::wrap(Token::Func, sl, sc, self.col-1, self.source_id),
	    "let"  => Spanned::wrap(Token::Let,  sl, sc, self.col-1, self.source_id),
	    "var"  => Spanned::wrap(Token::Var,  sl, sc, self.col-1, self.source_id),

	    _      => Spanned::wrap(Token::Identifier(slice), sl, sc, self.col-1, self.source_id),
	}
    }
    
    pub fn lex (&mut self) -> Vec<Spanned<Token<'source>>> {
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
		    tokens.push (Spanned::wrap (Token::OBrace, sl, sc, self.col-1, self.source_id));
		}
		'}' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::CBrace, sl, sc, self.col-1, self.source_id));
		}
		'(' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::OParen, sl, sc, self.col-1, self.source_id));
		}
		')' => {
		    self.advance();
		    tokens.push (Spanned::wrap (Token::CParen, sl, sc, self.col-1, self.source_id));
		}
		_ => {
		    self.diagnostics.push (Diag::InvalidCharacter (Span {
			file_id: self.source_id,
			line:    sl,
			col:     sc,
			cole:    self.col,
		    }, ch));
		    self.advance();
		}		
	    }
	}
	
	tokens.push (Spanned::wrap (Token::EOF, 0, 0, 0, self.source_id));
	tokens
    }
}

pub fn lex_source<'source> (source: SourceFile, smap: &'source SourceMap) -> Result<Vec<Spanned<Token<'source>>>, Vec<Diag<'source>>> {
    let mut lexer = Lexer::new(source, smap);
    let tokens = lexer.lex();

    if lexer.diagnostics.is_empty() {
	return Ok (tokens)
    }
    Err (lexer.diagnostics)
}
