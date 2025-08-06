#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::utils::string::*;
use crate::utils::array::*;
use crate:: {cogstr, dref};
use crate::frontend::arena::*;


pub enum HirItem {
    // Include (path, system)
    // path: path to file
    // system: used mainly for cbackend. it denotes whether to use <path> or "path"
    Include (CogString, bool),
    
    FunctionDef (*mut HirFuncDef),
}

pub struct HirFuncDef {
    name:      Option<CogString>,
    ret_type:  Option<HirType>,
}

pub enum HirType {
    // Integer (signed)
    // signed: denotes whether the integer is signed or unsigned
    Integer (bool),

    // Equivalent to 'void' in C
    None,
}

pub struct HirModule {
    arena: *mut Arena,
    
    items: *mut CogArray<HirItem>,
}

pub unsafe fn hir_module_new (arena: *mut Arena) -> *mut HirModule {
    let ptr = arena_alloc_ty::<HirModule>(arena);
    dref!(ptr).arena  = arena;
    dref!(ptr).items  = cog_arr_new(arena);
    ptr
}

pub unsafe fn hir_function (module: *mut HirModule) -> *mut HirFuncDef {
    let mut funcdef = arena_contruct(dref!(module).arena, HirFuncDef {
	name: None,
	ret_type: None,
    });
    
    let item = HirItem::FunctionDef(funcdef);
    return funcdef;
}
