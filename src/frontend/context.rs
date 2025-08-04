#![allow(unused)]

use crate::frontend::symbols:: {FunctionInfo, FunctionTable};

pub struct Context<'a> {
    functions: FunctionTable<'a>,
    parent:    Option<&'a mut Self>,
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<&'a mut Self>) -> Self {
	Self {
	    parent,
	    functions: FunctionTable::new(),
	}
    }
}
