#![allow(unused)]

use crate::frontend::symbols:: {FunctionInfo, FunctionTable};

use std::rc::Rc;
use std::cell::RefCell;

pub type SharedContext<'a> = Rc<RefCell<Context<'a>>>;
#[derive(Debug)]
pub struct Context<'a> {
    functions: FunctionTable<'a>,
    pub parent:    Option<SharedContext<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<SharedContext<'a>>) -> Self {
	Self {
	    parent,
	    functions: FunctionTable::new(),
	}
    }

    pub fn new_shared(parent: Option<SharedContext<'a>>) -> SharedContext<'a> {
	Rc::new (
	    RefCell::new (
		Self {
		    parent,
		    functions: FunctionTable::new(),
		}
	    )
	)
    }
    
    pub fn has_parent(&self) -> bool {
	return self.parent.is_some();
    }
    
    pub fn add_function(&mut self, name: &'a str, info: FunctionInfo<'a>) {
	self.functions.add(name, info);
    }
}
