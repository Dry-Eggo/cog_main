#![allow(unused)]
use std::io::Read;
use crate::dref;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::frontend::arena::*;

#[macro_export] macro_rules! dref {
    ($ptr: expr) => {
	(*$ptr)
    }
}

pub fn open_file(path: &String) -> Option<String> {
    let mut file = std::fs::File::open(path);
    match file {
	Ok(mut file) => {
	    let mut buffer = String::new();

	    if let Ok(x) = file.read_to_string(&mut buffer) {
		return Some(buffer);
	    }

	    None
	    
	},
	Err(err)    => {
	    return None;
	}
    }
}

pub fn string_to_lines(source: CogString, arena: *mut Arena) -> *mut CogArray<CogString> {
    unsafe {
	let arr = cog_arr_new(arena);
	let slice = std::slice::from_raw_parts(source.data, source.len);
	let slice_str = std::str::from_utf8_unchecked(slice);
	for s in slice_str.lines() {
	    cog_arr_push(arr, cogstr_new(s, arena));
	}
	arr
    }
}
