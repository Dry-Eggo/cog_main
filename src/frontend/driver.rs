#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::Args;
use crate::frontend::arena::*;
use crate::dref;
use crate::cogstr;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::utils::utils::*;
use crate::frontend::lexer::*;
use crate::frontend::parser::*;
use crate::frontend::error::*;


pub struct Driver {
    lexer:  *mut Lexer,
    parser: *mut Parser, 
    arena:  *mut Arena,
}

pub fn driver_run (args: Args) {
    unsafe {
	let mut arena  = arena_new(1024);
	let driver = arena_alloc_ty::<Driver>(&mut arena);
	dref!(driver).arena = &mut arena;
    
	let content_opt = open_file(&args.input_file);

	let source = {
	    if let Some(content) = content_opt {
		cogstr!(content, dref!(driver).arena)
	    } else {
		eprintln!("error: Unable to open file: {}", args.input_file);
		std::process::exit(1);
	    }
	};
	
	dref!(driver).lexer = lexer_new(dref!(driver).arena, source);
	lexer_lex(dref!(driver).lexer);
	let tokens = dref!(dref!(driver).lexer).tokens;	

	//dref!(driver).parser = parser_new(dref!(driver).arena, tokens);
	//if let Some(count) = parser_parse(dref!(driver).parser) {
	//    println!("Cog: {count} parsing error[s] occurred");
	//    report_syntax_errors(parser_get_errors(dref!(driver).parser), count);
	//}
	
	driver_free(driver);
    }
}

pub unsafe fn driver_free (driver: *mut Driver) {
    arena_free(dref!(driver).arena)
}
