#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::frontend::arena::*;
use crate::utils::map::Hashable;

use std::ptr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CogString {
    pub data: *mut u8,
    pub len:  usize
}

impl Hashable for CogString {
    fn hash(&self) -> usize {
	unsafe {
	    let string_ver = cogstr_to_str(*self);
	    djb2_hash(string_ver) as usize
	}
    }
}


// linK: https://mojoauth.com/hashing/bernsteins-hash-djb2-in-rust/
fn djb2_hash(input: &str) -> u64 {
    let mut hash: u64 = 5381; // Starting value for the hash
    for byte in input.bytes() {
        // Update the hash value with bitwise operations
        hash = (hash << 5).wrapping_add(hash).wrapping_add(byte as u64);
    }
    hash // Return the final hash value
}

pub unsafe fn cogstr_new (s: &str, arena: *mut Arena) -> CogString {
    return arena_alloc_str(arena, s);
}

#[macro_export] macro_rules! cogstr {
    ($str: literal, $arena: expr) => {
	(cogstr_new($str, $arena))
    };

    ($str: expr, $arena: expr) => {
	(cogstr_from_string($str, $arena))
    }
}

pub unsafe fn cogstr_from_string (input: String, arena: *mut Arena) -> CogString {
    let bytes = input.as_bytes();
    let len   = bytes.len();

    let dest = arena_alloc(arena, len);
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
