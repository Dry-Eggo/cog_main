#![allow(unused)]

use crate::frontend::token::{Span, Token};
use crate::frontend::driver:: {SourceFile, SourceMap};

pub type Diagnostics<'a> = Vec<Diag<'a>>;

#[derive(Debug, Clone)]
pub enum Diag<'a> {
    InvalidCharacter(/* location:  */ Span, /* char: */ char),
    UnexpectedTokenWithEx (UnexpectedTokenWithExSub<'a>),
    MissingIdentifier (Span, Token<'a>),
}

#[derive(Debug, Clone)]
pub struct UnexpectedTokenWithExSub<'a> {
    pub span: Span,
    pub expected: Token<'a>,
    pub got:      Token<'a>,
}

pub enum CompileError {
    LexingError,
    ParsingError,
    SemanticError,
    CodeGenError,
}

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

fn get_line_from_span<'a> (span: &Span, source: &'a String) -> &'a str {
    let lines = source.lines().collect::<Vec<_>>();
    let line  = span.line;
    let line_str = lines[line - 1];
    line_str
}

const DEFAULT_IDENTIFIER: &str = "foo";

pub fn report_errors (smap: &SourceMap, errors: Diagnostics) {
    for err in errors {
	if let Diag::InvalidCharacter (span, ch) = err {
	    let source   = smap.get_source_by_id (span.file_id);
	    let filename = match smap.get_filename (span.file_id) {
		Some (x) => x,
		_  => "invalid_path",
	    };
	    let line = get_line_from_span (&span, source);
	    println!("error: {}:{}: invalid character while lexing: {ch:?}", filename, span);
	    println!(" {:>5} |", "");
	    println!(" {:>5} |{}", span.line, line);
	    println!(" {:>5} |{}", "", span.underline(line));
	} else if let Diag::MissingIdentifier (span, prev_tok) = err {
	    let source   = smap.get_source_by_id (span.file_id);
	    let filename = match smap.get_filename (span.file_id) {
		Some (x) => x,
		_  => "invalid_path",
	    };
	    let line = get_line_from_span (&span, source);
	    println!("error: {}:{}: expected an identifier after '{}'", filename, span, prev_tok);
	    println!(" {:>5} |", "");
	    println!(" {:>5} |{}", span.line, line);
	    println!(" {:>5} |{}", "", span.underline(line));
	    println!(" hint:  try '{}'", format!("{} {DEFAULT_IDENTIFIER} {}", &line[..span.col-1], &line[span.col..]));
	}
    }
}
