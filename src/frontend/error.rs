#![allow(unsafe_op_in_unsafe_fn)]
use crate::frontend::arena::*;
use crate::frontend::token::*;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::dref;

pub struct SyntaxError {
    msg: CogString,
    hint: Option<CogString>,
    span: Span
}

pub unsafe fn new_syntax_error (span: Span, msg: &str, hint: Option<&str>, arena: *mut Arena) -> *mut SyntaxError {
    let err = arena_alloc_ty::<SyntaxError>(arena);
    dref!(err).msg = cogstr_new(msg, arena);
    if let Some(h) = hint {
	dref!(err).hint = Some(cogstr_new(h, arena));
    } else {
	dref!(err).hint = None;
    }    
    dref!(err).span = span;
    err
}

unsafe fn report_syntax_error (err: *mut SyntaxError) {
    
}

pub unsafe fn report_syntax_errors (errors: *mut CogArray<*mut SyntaxError>, count: usize) {
    for i in 0..count {
	let err = cog_arr_get(errors, i);
	report_syntax_error(*err)
    }
}
