#![allow(unused)]
mod frontend;
mod utils;

use frontend::{driver::*};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Dry-Eggo", version = "0.0.1", about = "The Cog Compiler", long_about = None)]
struct Args {
    #[command(subcommand)]
    command:      CompileCommand,
}

#[derive(Subcommand, Debug)]
enum CompileCommand {
    Build {	
	#[arg(name = "target")]
	target_path: Option<String>,

	#[arg(short = 'o', long = "output", name = "build_dir", default_value = "build")]
	output_path: String,
    },
    Run {
	#[arg(short = 'i', long = "input", name = "target", default_value = "./main.cg")]
	target_path: String,
    },
    NoCmd
}

fn main() {
    
    let mut compiler_options = Args::parse();

    unsafe { driver_run(compiler_options); }
    
    // let mut lexer = lexer::Lexer::new(source, "foo");
    // let tokens = lexer.lex();
}
