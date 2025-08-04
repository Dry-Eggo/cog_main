#![allow(unused)]

use crate::frontend::span:: {Spanned, Span};
use std::collections::HashMap;

pub struct FunctionInfo<'a> {
    name:        String,
    declaration: Span<'a>,
}

pub struct FunctionTable<'a> {
    functions: HashMap<String, Box<FunctionInfo<'a>>>,    
}

impl<'a> FunctionInfo<'a> {
    pub fn new(name: String, span: Span<'a>) -> Self {
	Self {
	    name,
	    declaration: span
	}
    }

    pub fn get_name(&self) -> &String {
	return &self.name;
    }
}

impl<'a> FunctionTable<'a> {
    pub fn new() -> Self {
	Self {
	    functions: HashMap::new(),
	}
    }
}
