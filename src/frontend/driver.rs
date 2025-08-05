#![allow(unused)]

use crate::{Args, CompileCommand, utils::utils};
use crate::frontend::parser::Parser;
use crate::frontend::lexer::Lexer;
use crate::frontend::semantics::Semantics;

pub struct Driver;

impl Driver {
    pub fn run(args: Args) -> Result<(), i32> {
	let mut arena = crate::frontend::arena::Allocator::new(0);
	if let CompileCommand::Build {target_path: ref inpath, output_path: ref outpath} = args.command {
	    if let Some(path) = inpath {
		let content = utils::open_file(path.to_string());
		if let Some(source) = content {
		    unsafe {
			let mut lexer: *mut Lexer = Lexer::new(&mut arena, &source, &path);
			let tokens = (*lexer).lex();
			let mut parser: *mut Parser = Parser::new(&tokens, &mut arena);
		    }
		    
		} else {
		    println!("Cog: unable to read file: '{path}'");
		    return Err(1);
		}
	    }
	}
	
	Ok(())
    }
    
}
