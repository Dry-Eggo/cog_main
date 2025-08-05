#![allow(unused)]

use crate::utils::string::*;

#[derive(Debug, Copy, Clone)]
pub struct Span {
    filename: CogString,
    line:     usize,
    cols:     usize,
    cole:     usize,
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


