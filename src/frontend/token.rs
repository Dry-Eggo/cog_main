#![allow(unused)]

use crate::utils::string::*;

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub filename: CogString,
    pub line:     usize,
    pub cols:     usize,
    pub cole:     usize,
}

impl Span {
    pub fn display(&self) -> String {
	unsafe { format!("{}:{}:{}", cogstr_to_str(self.filename), self.line, self.cols) }
    }

    pub fn merge(&self, other: Self) -> Self {
	Self {
	    filename: self.filename,
	    line:     self.line,
	    cols:     self.cols,
	    cole:     self.cole
	}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Identifier(CogString),
    Number    (CogString),

    Func, Let,

    Semi, OBrace, CBrace, OParen, CParen, Comma,

    Eof
}

pub fn span_new(filename: CogString, line: usize, cols: usize, cole: usize) -> Span {
    Span {
	filename,
	line,
	cols,
	cole,
    }
}

pub fn span_wrap<T>(span: Span, item: T) -> Spanned<T> {
    Spanned {
	item,
	span
    }
}


