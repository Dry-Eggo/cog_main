#![allow(unused)]

#[derive(Debug, Clone)]
pub struct Span<'a> {
    filename: &'a str,
    line:     usize,
    column:   usize,
    colend:   usize,
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
}

impl<'a, T> Spanned<'a, T> {
    pub fn span(item: T, span: Span<'a>) -> Self {
	Self {
	    item,
	    span
	}
    }
}
