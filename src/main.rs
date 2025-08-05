#![allow(unused)]
mod frontend;
mod utils;

use frontend::{driver::*};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Dry-Eggo", version = "0.0.1", about = "The Cog Compiler", long_about = None)]
struct Args {    
    #[arg(name  = "path to file")]
    input_file: String,
}

fn main() {
    
    let mut compiler_options = Args::parse();

    driver_run(compiler_options);
}
