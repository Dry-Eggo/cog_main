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

pub type RootNode = *mut CogArray<Option<SpannedItem>>;

pub struct Parser {
    cursor:     usize,
    max:        usize,
    arena:      *mut Arena,
    tokens:     *mut CogArray<Spanned<Token>>,

    pub root:   RootNode,
    errors:     *mut CogArray<*mut SyntaxError>
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
pub unsafe fn parser_parse (parser: *mut Parser) -> Option<usize> {

    while parser_now(parser).is_some() {
	if pmatch!(parser, Token::Func) {
	    let func = parse_function(parser);
	    cog_arr_push(dref!(parser).root, func);
	}
	parser_advance(parser);
    }
    
    let err_count = cog_arr_len(dref!(parser).errors);
    if err_count > 0 {
	Some(err_count)
    } else {
	None
    }
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

unsafe fn expect_identifer (parser: *mut Parser) -> Option<CogString> {
    if let Token::Identifier(name) = parser_now(parser).unwrap().item {
	parser_advance(parser);
	return Some(name);
    } else {
	error!(parser, "expected identifer before token", None, parser_now(parser).unwrap().span);
	None
    }	    
}

unsafe fn parse_function (parser: *mut Parser) -> Option<SpannedItem> {
    let function = arena_alloc_ty::<FunctionDef>(dref!(parser).arena);
    let start_span = parser_now(parser).unwrap().span;
    parser_advance(parser); // the 'fn' keyword

    let function_name = if let Some(name) = expect_identifer(parser) {
	name
    } else {
	cogstr!("error", dref!(parser).arena)
    };

    // TODO: parameters will be delayed till future stages
    expect(parser, Token::OParen);
    expect(parser, Token::CParen);

    let mut function_body = None;
    if (pmatch!(parser, Token::OBrace)) {
	parser_advance(parser);
	function_body = parse_body(parser);
	expect(parser, Token::CBrace);
    } else {
	todo!("Parse foward declaration")
    }

    let end_span = parser_now(parser).unwrap().span;    
    Some(span_wrap(start_span.merge(end_span), Item::FunctionDef(
	FunctionDef {
	    name: function_name,
	    body: function_body
	}
    )))
}

unsafe fn parse_body (parser: *mut Parser) -> Option<SpannedStmt> {
    let p = &mut *parser;
    let stmts = cog_arr_new(p.arena);
    let start_span = parser_now(parser).unwrap().span;

    while !pmatch!(parser, Token::CBrace) {
	let stmt = parse_stmt(parser);
    }
    
    let end_span = parser_now(parser).unwrap().span;    
    Some(span_wrap(start_span.merge(end_span), Stmt::CompoundStmt(stmts)))
}

unsafe fn parse_stmt (parser: *mut Parser) -> Option<SpannedStmt> {
    todo!()
}
