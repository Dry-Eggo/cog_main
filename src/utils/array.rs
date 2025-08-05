#![allow(unsafe_op_in_unsafe_fn)]
use std::mem;
use std::ptr;
use crate::frontend::arena::*;
use crate::dref;

pub struct CogArray <T> {
    ptr: *mut T,
    len: usize,
    cap: usize,

    arena: *mut Arena
}

pub unsafe fn cog_arr_new<T>(arena: *mut Arena) -> *mut CogArray<T> {
    let array = arena_alloc_ty::<CogArray<T>>(arena);
    dref!(array).ptr = ptr::null_mut();
    dref!(array).len = 0;
    dref!(array).cap = 0;
    dref!(array).arena = arena;
    array
}

pub unsafe fn cog_arr_len<T> (arr: *mut CogArray<T>) -> usize {
    dref!(arr).len
}

pub unsafe fn cog_arr_push<T>(arr: *mut CogArray<T>, value: T) {
    if dref!(arr).len == dref!(arr).cap {
	let new_cap = if dref!(arr).cap == 0 { 4 } else { dref!(arr).cap * 2 };
	let new_ptr = arena_alloc_array(dref!(arr).arena, new_cap);

	if !dref!(arr).ptr.is_null() {
	    ptr::copy_nonoverlapping(dref!(arr).ptr, new_ptr, dref!(arr).len);
	}

	dref!(arr).ptr = new_ptr;
	dref!(arr).cap = new_cap;
    }

    let dest = dref!(arr).ptr.add(dref!(arr).len);
    ptr::write(dest, value);
    dref!(arr).len += 1;
}

pub unsafe fn cog_arr_get<T>(arr: *mut CogArray<T>, n: usize) -> *mut T {
    if n >= dref!(arr).len {
	return ptr::null_mut();
    }

    dref!(arr).ptr.add(n)
}
