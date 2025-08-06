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
    pub name:      Option<CogString>,
    pub ret_type:  Option<HirType>,
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
    
    pub items: *mut CogArray<HirItem>,
}

pub unsafe fn hir_module_new (arena: *mut Arena) -> *mut HirModule {
    let ptr = arena_alloc_ty::<HirModule>(arena);
    dref!(ptr).arena  = arena;
    dref!(ptr).items  = cog_arr_new(arena);
    ptr
}


/// Returns a pointer to function item in the module
/// -----------------------------------------------
/// let func = hir_function(module);
/// hir_func_set_name(func, cogstr!("main", arena));
/// hir_func_set_return_type(func, ty);
/// hir_func_return_value(func, hir_expr);
/// ------------------------------------------------
pub unsafe fn hir_function (module: *mut HirModule) -> *mut HirFuncDef {
    let mut funcdef = arena_contruct(dref!(module).arena, HirFuncDef {
	name: None,
	ret_type: None,
    });
    
    let item = HirItem::FunctionDef(funcdef);
    cog_arr_push(dref!(module).items, item);
    return funcdef;
}

pub unsafe fn hir_func_set_name (func: *mut HirFuncDef, name: CogString) {
    dref!(func).name = Some(name);
}

pub unsafe fn hir_func_set_return_type (func: *mut HirFuncDef, ty: HirType) {
    dref!(func).ret_type = Some(ty);
}
