#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::frontend::token::*;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::{ dref, cogstr };

pub enum SymbolInfo {
    VariableInfo ( VarInfo ),
    FunctionInfo ( FuncInfo ),
}

pub struct VarInfo {
    name:             CogString,
    declaration_span: Span,
    is_mutable:       bool,
    is_initialized:   bool,
}

pub struct FuncInfo {
    pub name:             CogString,
    pub declaration_span: Span,
    pub param_count:      usize,
    /* Not Implemented yet */
    /* params:        CogArray<*mut ParamInfo> */
}

pub fn var_info_new (name: CogString, span: Span, is_mutable: bool, is_init: bool) -> VarInfo {
    VarInfo {
	name,
	declaration_span: span,
	is_mutable,
	is_initialized: is_init
    }
}

pub fn func_info_new (name: CogString, span: Span, param_count: usize) -> FuncInfo {
    FuncInfo {
	name,
	declaration_span: span,
	param_count
    }
}


// allocated version?
pub unsafe fn symbol_info_new (arena: *mut Arena, obj: SymbolInfo) -> *mut SymbolInfo {
    let ptr = arena_alloc_ty(arena);
    *ptr = obj;
    ptr
}
