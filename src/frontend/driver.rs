#![allow(unused)]

use crate::{Args, CompileCommand, utils::utils};
use crate::frontend::parser::Parser;
use crate::frontend::lexer::Lexer;

pub struct Driver;

impl Driver {
    pub fn run(args: Args) -> Result<(), i32> {

	if let CompileCommand::Build {target_path: ref inpath, output_path: ref outpath} = args.command {
	    if let Some(path) = inpath {
		let content = utils::open_file(path.to_string());
		if let Some(source) = content {

		    let mut lexer = Lexer::new(&source, &path);
		    let tokens    = lexer.lex();

		    let mut parser =  Parser::new(&source, &tokens);
		    let root       =  parser.build_ast();
		} else {
		    println!("Cog: unable to read file: '{path}'");
		    return Err(1);
		}
	    }
	}
	
	Ok(())
    }
    
}
