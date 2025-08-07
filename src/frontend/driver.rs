
use std::io::Read;
use crate::frontend::lexer::*;
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

	match Semantics::check (self, ast) {
	    Some (ref sema) => {
		if let Some (ref target) = self.args.backend {
		    match target {
			Backend::TargetNasm64 => {
			    let lirmod = LirContext::lower (&sema.irmod);
			    if let Some (mut nctx) = NasmContext::generate (&lirmod.unwrap()) {
				let generated_assembly = nctx.build_output();
				self.emit_file (generated_assembly);
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

    fn emit_file (&self, content: String) {
	// TODO: actually get output path from cli
	let _ = std::fs::write("a.s", &content);
	let nasm_cmd = std::process::Command::new ("nasm")
	    .arg("-felf64")
	    .arg("a.s")
	    .arg("-o")
	    .arg("a.o")
	    .arg("-g")
	    .arg("-F dwarf")
	    .status();
	let linker_cmd = std::process::Command::new ("gcc")
	    .arg("a.o")
	    .arg("-o")
	    .arg("a.out")
	    .status();
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
