#![allow(unused)]

use crate::frontend::span:: {Spanned, Span};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionInfo<'a> {
    pub name:        String,
    declaration: Span<'a>,
}

#[derive(Debug)]
pub struct FunctionTable<'a> {
    functions: HashMap<String, FunctionInfo<'a>>, 
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

    pub fn add(&mut self, name: String, info: FunctionInfo<'a>) {
	self.functions.insert(name, info);
    }

    pub fn get_mut(&mut self, query: &str) -> Option<&mut FunctionInfo> {
	self.functions.get_mut(query)
    }
}
