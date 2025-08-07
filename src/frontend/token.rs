use std::fmt::Display;

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
    Func,
    Var,
    Let,

    Identifier (&'a str),
    Integer,


    Eq,
    OBrace,
    CBrace,
    OParen,
    CParen,

    EOF
}

impl<'a> Display for Token<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
	match self {
	    Token::Func => write! (f, "fn"),
	    Token::Var  => write! (f, "var"),
	    Token::Identifier ( ident ) => write!(f, "Ident ( {} )", ident),
	    Token::OBrace => write! (f, "{{"),
	    Token::CBrace => write! (f, "}}"),
	    Token::OParen => write! (f, "("),
	    Token::CParen => write! (f, ")"),
	    Token::EOF    => write! (f, "<eof>"),
	    _ => todo!()
	}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    /* filename: &'a str, */
    line: usize,
    col:  usize,
    cole: usize
}

impl Span {
    pub fn merge (&self, other: &Self) -> Self {
	Self {
	    line: self.line,
	    col:  self.col,
	    cole: other.cole
	}
    }
}

#[derive(Clone, Copy)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn wrap (item: T, line: usize, col: usize, cole: usize) -> Self {
	Self {
	    item,
	    span: Span {
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
