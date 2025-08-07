
use std::io::Read;
use crate::frontend::lexer::*;
use crate::frontend::parser::Parser;
use crate::frontend::semantics::Semantics;
use crate::Args;

macro_rules! cog_error {
    ($($msg: tt)*) =>  {
	{
	    eprintln!("Cog Error: {}", format!($($msg)*));
	    std::process::exit(1)
	}
    }
}

pub enum CompileError {
    
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Driver {
    pub args:  Args,
    pub source: String,
}


impl Driver {
    pub fn new (args: Args) -> Self {
	let source = open_file_or_fail (&args.input_file);
	Self {
	    source,
	    args,
	}
    }

    pub fn run_compilation(&self) -> Result<(), CompileError> {
	let mut lexer = Lexer::new(self);
	let tokens = lexer.lex();
	let ast = {
	    let mut parser = Parser::new(self, &tokens);
	    parser.parse()
	};

	Semantics::check (self, ast);
	Ok (())
    }
}


pub fn open_file_or_fail (path: &str) -> String {
    let mut content = String::new();
    if let Ok (mut file) = std::fs::File::open(&path) {
	let _ = file.read_to_string(&mut content);
	return content
    }
    cog_error!("Unable to open file '{}'", path)
}
