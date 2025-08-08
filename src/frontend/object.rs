#![allow(unused)]
use std::collections::HashMap;
use crate::frontend::token::Span;

pub type FunctionId = usize;

#[derive(Debug, Clone)]
pub struct FunctionInfo<'source> {
    name: &'source str,
    span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionTable<'source> {
    entries: Vec<FunctionInfo<'source>>,
    map:     HashMap<&'source str, FunctionId>,
}

impl<'source> FunctionTable<'source> {
    pub fn new () -> Self {
	Self {
	    entries: vec![],
	    map:     HashMap::new()
	}
    }

    pub fn make_function (&mut self, name: &'source str, span: Span) -> FunctionId {
	if let Some ((id, prev)) = self.entries.iter().enumerate().find (|f| f.1.name == name) {
	    return id
	}
	
	let id = self.entries.len();
	self.entries.push (FunctionInfo{ name, span });
	id
    }

    /// if functions exists, perform 'f' on it mutably.
    /// else inserts 'func' into the table and returns the new id    
    pub fn get_and_or_insert (&mut self, id: FunctionId, f: fn (&mut FunctionInfo<'source>), func: FunctionInfo<'source>) -> Option<FunctionId> {
	if let Some(finfo) = self.get_mut (id) {
	    f (finfo);
	    return None
	}
	Some (self.add_function (func))
    }

    fn add_function (&mut self, func: FunctionInfo<'source>) -> FunctionId {
	let id = self.entries.len();
	self.entries.push(func);
	id
    }
    
    pub fn get_mut (&mut self, id: FunctionId) -> Option<&mut FunctionInfo<'source>> {
	self.entries.get_mut (id)
    }

    pub fn get (&self, id: FunctionId) -> Option<&FunctionInfo<'source>> {
	self.entries.get (id)
    }
}
