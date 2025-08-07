

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
    CParena,

    EOF
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    /* filename: &'a str, */
    line: usize,
    col:  usize,
    cole: usize
}

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
