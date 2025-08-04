#![allow(unused)]

use crate::frontend::symbols:: {FunctionInfo, FunctionTable};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Context<'a> {
    functions: FunctionTable<'a>,
    pub parent:    Option<Rc<RefCell<Self>>>,
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
	Self {
	    parent,
	    functions: FunctionTable::new(),
	}
    }

    pub fn has_parent(&self) -> bool {
	return self.parent.is_some();
    }
    
    pub fn add_function(&mut self, name: String, info: FunctionInfo<'a>) {
	self.functions.add(name, info);
    }
}
