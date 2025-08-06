#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::dref;
use crate::cogstr;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::utils::utils::*;
use crate::utils::map::*;

use crate::frontend::parser:: {Parser, RootNode};
use crate::frontend::ast::*;
use crate::frontend::ir::*;
use crate::frontend::token::  {Span};
use crate::frontend::objects::*;
use crate::frontend::error::*;


struct Context {
    parent: Option<*mut Context>,
    env:    *mut CogMap<CogString, *mut SymbolInfo>
}

unsafe fn context_new (
    parent: Option<*mut Context>,
    arena: *mut Arena
) -> *mut Context {    
    let ctx = arena_alloc_ty::<Context>(arena);
    dref!(ctx).parent = parent;
    dref!(ctx).env    = cogmap_new(arena);
    ctx
}

/// Returns old info if it already existed.
unsafe fn context_add (ctx: *mut Context, name: CogString, sym: *mut SymbolInfo) -> Option<*const SymbolInfo> {
    if let Some(old_ptr) = cogmap_insert(dref!(ctx).env, &name, &sym) {
	return Some(*old_ptr)
    }
    None
}

unsafe fn context_has (ctx: *mut Context, name: CogString) -> bool {
    if let Some(ptr) = cogmap_get(dref!(ctx).env, &name) {
	return true
    }
    false
}

unsafe fn context_get (ctx: *mut Context, name: CogString) -> Option<*const SymbolInfo> {
    if let Some(ptr) = cogmap_get(dref!(ctx).env, &name) {
	return Some(*ptr)
    }
    None
}

unsafe fn context_get_mut (ctx: *mut Context, name: CogString) -> Option<*mut SymbolInfo> {
    if let Some(ptr) = cogmap_get(dref!(ctx).env, &name) {
	return Some(*ptr as *mut _)
    }
    None
}
    
pub struct Semantics {
    root: RootNode,

    root_ctx: *mut Context,
    current_ctx: *mut Context,

    irmod:       *mut HirModule,
    
    arena: *mut Arena,	
}

pub unsafe fn semantics_new (root: RootNode, arena: *mut Arena) -> *mut Semantics {
    let sema = arena_alloc_ty::<Semantics>(arena);
    dref!(sema).root = root;

    dref!(sema).root_ctx    = context_new(None, arena);
    dref!(sema).current_ctx = dref!(sema).root_ctx;
    
    dref!(sema).arena = arena;    
    sema
}

pub unsafe fn semantics_get_module (sema: *mut Semantics) -> *mut HirModule {
    dref!(sema).irmod
}

/// Analyzes the parsed ast gotten from the parser
/// returns Option<(usize, usize)> representing (Warning Count, Error Count) or None
pub unsafe fn semantics_analyze_root (sema: *mut Semantics) -> Option<()> {
    // TODO: we do not support errors here for now
    // so we return Option<()>. will be changed.

    semantics_run_first_pass(sema);
    
    None
}

unsafe fn semantics_run_second_pass (sema: *mut Semantics) {
    let root = dref!(sema).root;
    for n in 0..cog_arr_len(root) {
	let item = cog_arr_get(root, n);

	if let Some(ref mut spanned) = *item {
	    if let Item::FunctionDef (ref mut function_def) = spanned.item {
		analyze_function(sema, function_def);
	    }
	}
    }    
}

unsafe fn semantics_run_first_pass (sema: *mut Semantics) {
    let root = dref!(sema).root;
    for n in 0..cog_arr_len(root) {
	let item = cog_arr_get(root, n);

	if let Some(ref mut spanned) = *item {
	    if let Item::FunctionDef (ref mut function_def) = spanned.item {
		register_function(sema, function_def, spanned.span);
	    }
	}
    }    
}

unsafe fn register_function (sema: *mut Semantics, func: *mut FunctionDef, span: Span) {
    let function_def = &mut *func;
    let arena        = dref!(sema).arena;
    
    let info = symbol_info_new(arena, SymbolInfo::FunctionInfo (func_info_new(function_def.name, span, 0)));
    context_add(dref!(sema).root_ctx, function_def.name, info);
}

unsafe fn analyze_function (sema: *mut Semantics, func: *mut FunctionDef) {
    let function_def = &mut *func;
    let arena        = dref!(sema).arena;
    
    match context_get(dref!(sema).root_ctx, function_def.name) {
	Some (syminfo) => {
	    match *syminfo {
		SymbolInfo::FunctionInfo (ref finfo) => {
		    let function = hir_function(dref!(sema).irmod);
		    hir_func_set_name(function, finfo.name);

		    if cogstr_to_str(finfo.name) == "main" {
			hir_func_set_return_type(function, HirType::Integer(true));
		    }
		}
		_ => unreachable!()
	    }
	}
	_ => unreachable!()
    }
}
