#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::Args;
use crate::frontend::arena::*;
use crate::utils::string::*;
use crate::frontend::lexer::*;

pub struct Driver {
    lexer: *mut Lexer,
    arena: *mut Arena,
}

pub unsafe fn driver_run (args: Args) {    
    let mut arena  = arena_new(1024);
    let driver = arena_alloc_ty::<Driver>(&mut arena);
    (*driver).arena = &mut arena;
    
    let source = cogstr_new("func main() {\n\tlet x = 40\n}", (*driver).arena);    
    (*driver).lexer = lexer_new((*driver).arena, source);
}
