#![allow(unused)]

use crate::utils::string::*;

pub struct Span {
    filename: CogString,
    line:     usize,
    cols:     usize,
    cole:     usize,
}

pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}


pub enum Token {
    Identifier(CogString),
    Number    (CogString),

    Func, Let,

    Semi, OBrace, CBrace, OParen, CParen, Comma
}

pub fn span_new(filename: CogString, line: usize, cols: usize, cole: usize) -> Span {
    Span {
	filename,
	line,
	cols,
	cole,
    }
}

pub fn span<T>(span: Span, item: T) -> Spanned<T> {
    Spanned {
	item,
	span
    }
}


