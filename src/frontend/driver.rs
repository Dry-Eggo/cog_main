
use std::io::Read;

use crate::frontend::lexer:: {Lexer};
use crate::frontend::error:: {CompileError};
use crate::frontend::parser::Parser;
use crate::frontend::semantics::Semantics;
use crate:: {Args, Backend};
use crate::backend::nasm64_backend::*;
use crate::backend::lir_unit::*;

macro_rules! cog_error {
    ($($msg: tt)*) =>  {
	{
	    eprintln!("Cog Error: {}", format!($($msg)*));
	    std::process::exit(1)
	}
    }
}

pub fn run_compilation(args: Args) -> Result<(), CompileError> {
    let source = open_file_or_fail (&args.input_file);
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex();
    let ast = {
	let mut parser = Parser::new(&tokens);
	parser.parse()
    };
    match Semantics::check (ast) {
	Some (ref sema) => {
	    if let Some (ref target) = args.backend {
		match target {
		    Backend::TargetNasm64 => {
			let lirmod = LirContext::lower (&sema.irmod);
			if let Some (mut nctx) = NasmContext::generate (&lirmod.unwrap()) {
			    let generated_assembly = nctx.build_output();
			    emit_file (generated_assembly);
			}
		    }
		    _ => todo!("backend not supported yet")
		}
	    }
	}
	None => {
	    todo! ("Report Errors")
	}
    }
    Ok (())
}

fn emit_file (content: String) {
    // TODO: actually get output path from cli
    let _ = std::fs::write("a.s", &content);
    let _ = std::process::Command::new ("nasm")
	.arg("-felf64")
	.arg("a.s")
	.arg("-o")
	.arg("a.o")
	.arg("-g")
	.arg("-F dwarf")
	.status();
    let _ = std::process::Command::new ("gcc")
	.arg("a.o")
	.arg("-o")
	.arg("a.out")
	.status();
}



pub fn open_file_or_fail (path: &str) -> String {
    let mut content = String::new();
    if let Ok (mut file) = std::fs::File::open(&path) {
	let _ = file.read_to_string(&mut content);
	return content
    }
    cog_error!("Unable to open file '{}'", path)
}

