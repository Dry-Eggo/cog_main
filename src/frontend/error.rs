#![allow(unused)]

use crate::frontend::token::Span;

pub struct SyntaxError {
    pub msg: String,
    pub span: Span,
    pub hint: Option<String>,
}

impl SyntaxError {
    pub fn new (msg: String, span: Span) -> Self {
	Self {
	    msg,
	    span,
	    hint: None,
	}
    }

    pub fn hinted (msg: String, span: Span, hint: String) -> Self {
	Self {
	    msg,
	    span,
	    hint: Some(hint),
	}
    }
}
