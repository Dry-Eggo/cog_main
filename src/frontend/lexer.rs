#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::utils::string::*;

pub struct Lexer {
    source: CogString,
    pos:    usize,
    line:   usize,
    col:    usize,
    arena:  *mut Arena
}

pub unsafe fn lexer_new(arena: *mut Arena, source: CogString) -> *mut Lexer {
    let lexer_ptr = arena_alloc_ty::<Lexer>(arena);
    let lexer = &mut *lexer_ptr;

    lexer.source = source;
    lexer.pos    = 0;
    lexer.line   = 1;
    lexer.col    = 1;
    lexer.arena  = arena;
    
    lexer_ptr
}


pub unsafe fn lexer_now(lexer: *mut Lexer) -> Option<char> {
    return cogstr_at((*lexer).source, (*lexer).pos);
}

pub unsafe fn lexer_peek(lexer: *mut Lexer, n: usize) -> Option<char> {
    return cogstr_at((*lexer).source, (*lexer).pos + n);    
}

pub unsafe fn lexer_advance(lexer: *mut Lexer) -> Option<char> {
    if let Some(ch) = lexer_now(lexer) {
	if ch == '\n' {
	    (*lexer).line += 1;
	    (*lexer).col   = 1;	    
	} else {
	    (*lexer).col  += 1;	    
	}
	(*lexer).pos += 1;
	return Some(ch)
    }
    None
}

pub unsafe fn lexer_lex(lexer: *mut Lexer) {
    while lexer_now(lexer).is_some() {
	if lexer_now(lexer).unwrap().is_whitespace() {
	    lexer_advance(lexer);
	    continue;
	}

	if lexer_now(lexer).unwrap().is_alphabetic() {
	    
	}
    }
}
