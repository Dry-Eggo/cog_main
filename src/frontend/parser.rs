#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::frontend::ast::*;
use crate::frontend::token::*;
use crate::frontend::error::*;
use crate::dref;
use crate::cogstr;
use crate::utils::string::*;
use crate::utils::array::*;

pub struct Parser {
    cursor: usize,
    max:    usize,
    arena:  *mut Arena,
    tokens: *mut CogArray<Spanned<Token>>,

    pub root:   *mut CogArray<SpannedItem>,
    errors: *mut CogArray<*mut SyntaxError>
}


pub unsafe fn parser_new  (arena: *mut Arena, tokens: *mut CogArray<Spanned<Token>>) -> *mut Parser {
    let parser = arena_alloc_ty::<Parser>(arena);
    dref!(parser).cursor = 0;
    dref!(parser).arena  = arena;
    dref!(parser).tokens = tokens;
    dref!(parser).max    = cog_arr_len(tokens);
    dref!(parser).root   = cog_arr_new(arena);
    dref!(parser).errors = cog_arr_new(arena);
    parser
}

macro_rules! pmatch {
    ($parser: expr, $tok: expr) => {
	(parser_now($parser).unwrap().item == $tok)
    }
}

macro_rules! error {
    ($parser: expr, $msg: expr, $hint: expr, $span: expr) => {
	parser_add_error($parser, $msg, $hint, $span)
    }
}

///  parses the tokens into a list of Items which represent top-level constructs in Cog
///  returns an Option<usize>.
///        set to None on 'No Error' or Some (err_count) if error was encountered while parsing
pub unsafe fn parser_parse (parser: *mut Parser) -> bool {

    while parser_now(parser).is_some() {
	if pmatch!(parser, Token::Func) {
	    parse_function(parser);
	}
	parser_advance(parser);
    }
    
    cog_arr_len(dref!(parser).errors) == 0
}

unsafe fn parser_add_error (parser: *mut Parser, msg: &str, hint: Option<&str>, span: Span) {
    cog_arr_push(dref!(parser).errors, new_syntax_error(span, msg, hint, dref!(parser).arena));
}

pub unsafe fn parser_get_errors (parser: *mut Parser) -> *mut CogArray<*mut SyntaxError>{
    dref!(parser).errors
}

unsafe fn parser_now (parser: *mut Parser) -> Option<Spanned<Token>> {
    if dref!(parser).cursor >= dref!(parser).max {
	return None
    }
    
    Some(*cog_arr_get(dref!(parser).tokens, dref!(parser).cursor))
}

unsafe fn parser_peek (parser: *mut Parser, ahead: usize) -> Option<Spanned<Token>> {
    if dref!(parser).cursor + ahead >= dref!(parser).max {
	return None
    }
    
    Some(*cog_arr_get(dref!(parser).tokens, dref!(parser).cursor + ahead))
}

unsafe fn parser_advance (parser: *mut Parser) {
    dref!(parser).cursor += 1;
}

unsafe fn parser_before (parser: *mut Parser) -> Option<Spanned<Token>> {
    if dref!(parser).cursor <= 0 {
	return None
    }
    
    Some(*cog_arr_get(dref!(parser).tokens, dref!(parser).cursor - 1))
}

unsafe fn expect (parser: *mut Parser, tok: Token) -> bool {
    if pmatch!(parser, tok) {
	parser_advance(parser);
	return true;
    }
    
    false
}

unsafe fn parse_function (parser: *mut Parser) {
    let function = arena_alloc_ty::<FunctionDef>(dref!(parser).arena);
    parser_advance(parser); // the 'fn' keyword

    let function_name = {
	if let Token::Identifier(name) = parser_now(parser).unwrap().item {
	    name
	} else {
	    error!(parser, "expected name after 'fn", None, parser_now(parser).unwrap().span);
	    cogstr!("error", dref!(parser).arena)
	}	
    };
}
