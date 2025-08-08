
use std::io::Read;
use std::collections::HashMap;

use crate::frontend::lexer:: {lex_source};
use crate::frontend::error:: {CompileError, report_errors};
use crate::frontend::parser::{parse_tokens};
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

pub fn abort_compilation (err: CompileError) -> ! {
    let (phase, code) = match err {
	CompileError::LexingError   => ("Lexing", 105),
	CompileError::ParsingError  => ("Parsing", 233),
	CompileError::SemanticError => ("Semantics", 142),
	_ => todo!(),
    };

    eprintln!("fatal: compilation aborted at {} phase", phase);    
    std::process::exit (code);
}

pub type SourceFile = usize;

#[derive(Debug, Clone)]
pub struct SourceMap {
    id:      HashMap<String, SourceFile>,
    sources: Vec<String>,
}

impl SourceMap {
    pub fn new () -> Self {
	Self {
	    id: HashMap::new(),
	    sources: vec![],
	}
    }

    pub fn make_source_file (&mut self, path: String) -> SourceFile {
	let id = self.sources.len();
	let source = open_file_or_fail (&path);
	self.id.insert (path, id);
	self.sources.push (source);
	id
    }

    pub fn get_source_by_id (&self, id: SourceFile) -> &String {
	self.sources.get (id).unwrap()
    }

    pub fn get_filename (&self, id: SourceFile) -> Option<&String> {
	let key = self.id.iter().find (|(key, id_)| {
	    **id_ == id
	});
	
	if let Some ((key, _)) = key {
	    return Some(key)
	}
	None
    }
}

pub fn run_compilation(args: Args) -> Result<(), CompileError> {
    let mut smap   = SourceMap::new();
    let source = smap.make_source_file (args.input_file);
    let tokens = lex_source (source, &smap);
    let Ok (tokens) = tokens else {
	report_errors (&smap, tokens.err().unwrap());
	abort_compilation(CompileError::LexingError)
    };
    
    let ast = {
	let mut items = parse_tokens(&tokens);
	let Ok (items) = items else {
	    report_errors (&smap, items.err().unwrap());
	    abort_compilation(CompileError::ParsingError)
	};
	items
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
	    todo! ("use default backend")
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

