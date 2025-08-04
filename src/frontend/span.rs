#![allow(unused)]

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub filename: &'a str,
    pub line:     usize,
    pub column:   usize,
    pub colend:   usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<'a, T> {
    pub item: T,
    pub span: Span<'a>
}


impl<'a> Span<'a> {
    pub fn new(filename: &'a str, line: usize, col: usize, cole: usize) -> Self {
	Self {
	    filename,
	    line,
	    column: col,
	    colend: cole
	}
    }

    pub fn display(&self) -> String {
	return format!("{}:{}:{}", self.filename, self.column, self.colend);
    }

    pub fn merge(s1: Span<'a>, s2: Span<'a>) -> Span<'a> {
	Self {
	    filename: s1.filename,
	    line:     s1.line,
	    column:   s1.column,
	    colend:   s2.colend,
	}
    }
}

impl<'a, T> Spanned<'a, T> {
    pub fn span(item: T, span: Span<'a>) -> Self {
	Self {
	    item,
	    span
	}
    }
}
