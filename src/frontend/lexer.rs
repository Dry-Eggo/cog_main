#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::frontend::token::*;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::{ dref, cogstr };

pub struct Lexer {
    source: CogString,
    filename: CogString,
    
    pos:    usize,
    line:   usize,
    col:    usize,
    arena:  *mut Arena,

    pub tokens: *mut CogArray<Spanned<Token>>
}

pub unsafe fn lexer_new(arena: *mut Arena, source: CogString, path: CogString) -> *mut Lexer {
    let lexer_ptr = arena_alloc_ty::<Lexer>(arena);
    let lexer = &mut *lexer_ptr;

    lexer.source = source;
    lexer.filename = path;
    
    lexer.pos    = 0;
    lexer.line   = 1;
    lexer.col    = 1;
    lexer.arena  = arena;
    lexer.tokens = cog_arr_new(arena);
    
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

pub unsafe fn lexer_add_token(lexer: *mut Lexer, tok: Spanned<Token>) {
    cog_arr_push(dref!(lexer).tokens, tok);
}

pub unsafe fn lexer_lex(lexer: *mut Lexer) {
    while lexer_now(lexer).is_some() {
	if lexer_now(lexer).unwrap().is_whitespace() {
	    lexer_advance(lexer);
	    continue;
	}

	if lexer_now(lexer).unwrap().is_alphabetic() {
	    let sl = (*lexer).line;
	    let sc  = (*lexer).col;
	    let mut buffer = String::new();
	    while lexer_now(lexer).is_some() && lexer_now(lexer).unwrap().is_alphanumeric()
		|| lexer_now(lexer).unwrap() == '_' {		    
		    buffer.push(lexer_advance(lexer).unwrap());
		}

	    let kind = match buffer.as_str() {
		"fn" => Token::Func,
		 _   => Token::Identifier(cogstr!(buffer, dref!(lexer).arena))
	    };
	    
	    let token = span_wrap(span_new(dref!(lexer).filename, sl, sc, dref!(lexer).col - 1), kind);
	    lexer_add_token(lexer, token);
	    continue;
	}

	let sl = dref!(lexer).line;
	let sc = dref!(lexer).col;
	match lexer_now(lexer).unwrap() {
	    '(' => {
		lexer_advance(lexer);
		let token = span_wrap(span_new(dref!(lexer).filename, sl, sc, dref!(lexer).col - 1), Token::OParen);
		lexer_add_token(lexer, token);
	    }
	    ')' => {
		lexer_advance(lexer);
		let token = span_wrap(span_new(dref!(lexer).filename, sl, sc, dref!(lexer).col - 1), Token::CParen);
		lexer_add_token(lexer, token);
	    }
	    '{' => {
		lexer_advance(lexer);
		let token = span_wrap(span_new(dref!(lexer).filename, sl, sc, dref!(lexer).col - 1), Token::OBrace);
		lexer_add_token(lexer, token);
	    }
	    '}' => {
		lexer_advance(lexer);
		let token = span_wrap(span_new(dref!(lexer).filename, sl, sc, dref!(lexer).col - 1), Token::CBrace);
		lexer_add_token(lexer, token);
	    }
	    c   => {
		lexer_advance(lexer);
		println!("Unexpected char: '{}'", c);
	    }
	}
    }
    
    let token = span_wrap(span_new(dref!(lexer).filename, 0, 0, 0), Token::Eof);
    lexer_add_token(lexer, token);
}
