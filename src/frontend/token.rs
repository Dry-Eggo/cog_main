use std::fmt::Display;
use crate::driver::SourceFile;

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
    Func,
    Var,
    Let, Mut,

    Identifier (&'a str),
    PrimitiveType (&'a str),
    Integer,


    Eq,
    OBrace,
    CBrace,
    OParen,
    CParen,
    Colon,
    SemiColon,

    EOF
}

impl<'a> Display for Token<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
	match self {
	    Token::Func => write! (f, "fn"),
	    Token::Var  => write! (f, "var"),
	    Token::Identifier ( ident ) => write!(f, "identifier {}", ident),
	    Token::PrimitiveType ( ty_str ) => write!(f, "primitive-type {}", ty_str),
	    Token::OBrace => write! (f, "{{"),
	    Token::CBrace => write! (f, "}}"),
	    Token::OParen => write! (f, "("),	    
	    Token::CParen => write! (f, ")"),
	    Token::Colon => write! (f, ":"),
	    Token::SemiColon => write! (f, ";"),
	    Token::EOF    => write! (f, "<eof>"),
	    _ => todo!()
	}
    }
}

impl Display for Span {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
	write!(f, "{}:{}-{}", self.line, self.col, self.cole)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub file_id: SourceFile,
    pub line: usize,
    pub col:  usize,
    pub cole: usize
}

impl Span {
    pub fn merge (&self, other: &Self) -> Self {
	Self {
	    file_id: self.file_id,
	    line: self.line,
	    col:  self.col,
	    cole: other.cole
	}
    }

    pub fn underline (&self) -> String {	
	if self.col > self.cole {
	    return String::new();
	}

	let mut result = " ".repeat(self.col-1);
	let width  = if self.col == self.cole { 1 } else { self.cole - self.col };
	result += "^";
	result.push_str(&"~".repeat(width-1));
	result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn wrap (item: T, line: usize, col: usize, cole: usize, id: SourceFile) -> Self {
	Self {
	    item,
	    span: Span {
		file_id: id,
		line,
		col,
		cole
	    }
	}
    }

    pub fn create (item: T, span: Span) -> Self {
	Self {
	    item,
	    span
	}
    }
}
