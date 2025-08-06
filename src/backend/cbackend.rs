
#![allow(unsafe_op_in_unsafe_fn)]
use crate::frontend::ir::*;
use crate::frontend::arena::*;
use crate:: {dref};
use crate::utils::array::*;
use crate::utils::string::*;

pub struct CContext {
    final_output: Option<String>,
    includes:     String,
    body:         String,

    irmod:        *mut HirModule,
}

pub unsafe fn cctx_new (arena: *mut Arena, irmod: *mut HirModule) -> *mut CContext {
    let ptr = arena_contruct(arena, CContext {
	final_output: None,
	includes: String::new(),
	body:     String::new(),
	irmod,
    });

    ptr
}

pub unsafe fn cctx_generate (cctx: *mut CContext) -> bool {
    let context = &mut *cctx;
    let items = dref!(context.irmod).items;
    for i in 0..cog_arr_len(items) {
	let item = cog_arr_get(items, i);
	match *item {
	    HirItem::FunctionDef (ref fdef) => {
		cctx_walk_function(cctx, /* whaaa!? */ &**fdef);
	    }
	    _ => todo!()
	}
    }
    
    true
}

pub unsafe fn cctx_walk_function (cctx: *mut CContext, func: &HirFuncDef) {
    todo!()
}

pub unsafe fn cctx_walk_type (cctx: *mut CContext, ty: &Option<HirType>) -> String {
    todo!()
}
