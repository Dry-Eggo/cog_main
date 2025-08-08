use clap::{Parser, Subcommand};

mod frontend;
mod backend;

use crate::frontend::driver;

#[derive(Parser, Debug, Clone)]
#[command(author = "Dry-Eggo", version = "0.0.1", about = "The Cog Compiler", long_about = None)]
struct Args {    

    #[arg(long, value_enum)]
    backend: Option<Backend>,
    
    #[command(subcommand)]
    command: Option<Command>,
   
    #[arg(name  = "path to file")]
    input_file: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[command(about = "Build Project")]
    Build,
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum Backend {
    TargetC,
    TargetLlvm,
    TargetNasm64,
}

fn main() {   
    let compiler_options = Args::parse();
    let _ = driver::run_compilation(compiler_options);
}
