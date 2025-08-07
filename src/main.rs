use clap::{Parser, /*Subcommand */ };

mod frontend;

use crate::frontend::driver::Driver;

#[derive(Parser, Debug, Clone)]
#[command(author = "Dry-Eggo", version = "0.0.1", about = "The Cog Compiler", long_about = None)]
struct Args {    
    #[arg(name  = "path to file")]
    input_file: String,
}

fn main() {   
    let compiler_options = Args::parse();
    let _ = Driver::new(compiler_options).run_compilation();
}
