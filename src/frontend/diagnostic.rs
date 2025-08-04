#![allow(unused)]

use crate::frontend:: {span::Span};

pub struct SyntaxError<'a> {
    span: Span<'a>,
    msg:  String,
    hint: Option<String>
}

pub struct DiagnosticEngine<'a> {
    syntax_errors: Vec<SyntaxError<'a>>
}

impl<'a> SyntaxError<'a> {
    pub fn new(msg: String, span: Span<'a>) -> Self {
	Self {
	    msg,
	    span,
	    hint: None
	}
    }
    
    pub fn new_hinted(msg: String, hint: String, span: Span<'a>) -> Self {
	Self {
	    msg,
	    span,
	    hint: Some(hint),
	}
    }
}

impl<'a> DiagnosticEngine<'a> {
    pub fn new () -> Self {
	Self {
	    syntax_errors: vec![]
	}
    }

    pub fn add_syntax_error(&mut self, err: SyntaxError<'a>) {
	self.syntax_errors.push(err);
    }
}
