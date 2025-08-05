#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use std::ptr;

#[derive(Copy, Clone, Debug)]
pub struct CogString {
    pub data: *mut u8,
    pub len:  usize
}

pub unsafe fn cogstr_new (s: &str, arena: *mut Arena) -> CogString {
    return arena_alloc_str(arena, s);
}

pub unsafe fn cogstr_from_string (input: String, arena: *mut Arena) -> CogString {
    let bytes = input.as_bytes();
    let len   = bytes.len();

    let dest = arena_alloc_align(arena, len, std::mem::align_of::<u8>());
    ptr::copy_nonoverlapping(bytes.as_ptr(), dest, len);
    
    CogString {
	data: dest,
	len,
    }
}

pub unsafe fn cogstr_to_str (s: CogString) -> &'static str {
    let slice = std::slice::from_raw_parts(s.data, s.len);
    std::str::from_utf8_unchecked(slice)
}

pub unsafe fn cogstr_at (s: CogString, n: usize) -> Option<char> {
    let string = cogstr_to_str(s);
    string.chars().nth(n)
}

pub unsafe fn cogstr_eq (s1: CogString, s2: CogString) -> bool {
    if s1.len != s2.len {
	return false;
    }

    std::slice::from_raw_parts(s1.data, s1.len) == std::slice::from_raw_parts(s2.data, s2.len)
}
