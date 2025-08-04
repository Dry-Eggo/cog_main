#![allow(unused)]
use std::io::Read;

pub fn open_file(path: String) -> Option<String> {
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
