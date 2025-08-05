#![allow(unused)]

use crate::frontend::span:: {Spanned, Span};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    name:        &'a str,
    declaration: Span<'a>,
}

#[derive(Debug)]
pub struct FunctionTable<'a> {
    functions: HashMap<&'a str, FunctionInfo<'a>>, 
}

impl<'a> FunctionInfo<'a> {
    pub fn new(name: &'a str, span: Span<'a>) -> Self {
	Self {
	    name,
	    declaration: span
	}
    }

    pub fn get_name(&self) -> &'a str {
	return self.name;
    }
}

impl<'a> FunctionTable<'a> {
    pub fn new() -> Self {
	Self {
	    functions: HashMap::new(),
	}
    }

    pub fn add(&mut self, name: &'a str, info: FunctionInfo<'a>) {
	self.functions.insert(name, info);
    }
}
