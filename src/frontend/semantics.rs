#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::dref;
use crate::cogstr;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::utils::utils::*;

use crate::frontend::parser:: {Parser, RootNode};
use crate::frontend::ast::*;
use crate::frontend::error::*;

pub struct Semantics {
    root: RootNode,
    arena: *mut Arena
}

pub unsafe fn semantics_new (root: RootNode, arena: *mut Arena) -> *mut Semantics {
    let sema = arena_alloc_ty::<Semantics>(arena);
    dref!(sema).root = root;
    dref!(sema).arena = arena;
    sema
}

/// Analyzes the parsed ast gotten from the parser
/// returns Option<(usize, usize)> representing (Warning Count, Error Count) or None
pub unsafe fn semantics_analyze_root (sema: *mut Semantics) -> Option<()> {
    // TODO: we do not support errors here for now
    // so we return Option<()>. will be changed.

    semantics_run_first_pass(sema);
    
    None
}

unsafe fn semantics_run_first_pass (sema: *mut Semantics) {
    let root = dref!(sema).root;
    for n in 0..cog_arr_len(root) {
	let item = cog_arr_get(root, n);

	if let Some(ref mut spanned) = *item {
	    if let Item::FunctionDef (ref mut function_def) = spanned.item {
		register_function(sema, function_def);
	    }
	}
    }    
}

unsafe fn register_function (sema: *mut Semantics, func: *mut FunctionDef) {
    let function_def = &mut *func;
    println!("Function: {}", cogstr_to_str(function_def.name));
}
